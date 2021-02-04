//"5842.77*0.75+(5842.77*0.1-230.20*12)" Inclusao CPAS rendimento global
//transição base incidência de 75% para 90%:
//rendimento global * 0.15 - (4104 + despesas afectas actividade) = X
//Se X > 0, rendimento global += X (i.e. o excesso é tributado a 90% em vez de 75%)

#![feature(const_fn)]
#![feature(trait_alias)]
#![feature(non_ascii_idents)]
//#![feature(specialization)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use std::io::Write;

#[macro_use]
extern crate failure;
use failure::Error;

pub mod irs;
pub mod rendimentos;
pub mod ss;

type Money = f64;
type MoneyRate = f64;
type MoneyYearly = f64;
type MoneyMonthly = f64;
type MoneyDaily = f64;
type MoneyDaily22 = f64;
type MoneyDaily30 = f64;
type Tax = f64;

fn diferenca_positiva(a: Money, b: Money) -> Money {
    (a - b).max(0.0)
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ElementoFamiliar {
    casado: bool,
    titular_unico: bool,
    dependentes: usize,
}

const DIAS_POR_MES: u8 = 22;

const IAS: MoneyMonthly = 435.76; // 2019
const salario_minimo: MoneyMonthly = 600.0; // 2019

const tax_seguro: Tax = 0.01;

fn bruto2liquido(bruto: MoneyMonthly, alimentacao: MoneyMonthly, dependentes: usize) -> MoneyRate {
    bruto * (1.0 - ss::tax::trabalhador - irs::retencao::tax(bruto, dependentes)) + alimentacao
}

fn liquido2bruto(
    mut liquido: MoneyRate,
    alimentacao: MoneyRate,
    dependentes: usize,
) -> Result<Vec<MoneyRate>, Error> {
    liquido -= alimentacao;
    let bruto = |tax_irs| liquido / (1.0 - ss::tax::trabalhador - tax_irs);
    let mut result = vec![];
    let mut old_l0 = 0.0;
    for l in irs::retencao::dependente_nao_casado.iter() {
        let b = bruto(l.1[std::cmp::min(5, dependentes)] / 100.0);
        //if b <= old_l0 { break; }
        if l.0 >= b && b >= salario_minimo && b > old_l0 {
            result.push(b);
        }
        old_l0 = l.0;
    }
    ensure!(!result.is_empty(), "No match for {}€", liquido);
    Ok(result)
}

fn bruto2empresa(bruto: MoneyRate, subsidio_alimentacao_mensal: MoneyRate) -> MoneyRate {
    bruto * (1.0 + ss::tax::empresa + 0.01) * 14.0 / 12.0 + subsidio_alimentacao_mensal
}

fn empresa2bruto(disponivel: MoneyRate, subsidio_alimentacao_mensal: MoneyRate) -> MoneyRate {
    (disponivel - subsidio_alimentacao_mensal) / (1.0 + ss::tax::empresa + 0.01) / 14.0 * 12.0
}

#[derive(Clone, Copy, Debug)]
enum SegSocVarCatB {
    M25 = -25,
    M20 = -20,
    M15 = -15,
    M10 = -10,
    M05 = -5,
    Zero = 0,
    P05 = 5,
    P10 = 10,
    P15 = 15,
    P20 = 20,
    P25 = 25,
}

struct Rendimentos {
    cat_a: MoneyYearly,
    meses_cat_a: f64,
    cat_b: MoneyYearly,
    meses_cat_b: f64,
    cat_b_opt: SegSocVarCatB,
    subsidio_alimentacao: MoneyMonthly,
}

impl Rendimentos {
    fn new<A: Into<MoneyYearly>, B: Into<MoneyYearly>, S: Into<MoneyMonthly>>(
        cat_a: A,
        meses_cat_a: f64,
        cat_b: B,
        meses_cat_b: f64,
        subsidio_alimentacao: S,
        cat_b_opt: SegSocVarCatB,
    ) -> Result<Rendimentos, Error> {
        Ok(Rendimentos {
            cat_a: cat_a.into(),
            meses_cat_a,
            cat_b: cat_b.into(),
            meses_cat_b,
            subsidio_alimentacao: subsidio_alimentacao.into(),
            cat_b_opt,
        })
    }

    fn liquido_cat_a(&self) -> Money {
        self.cat_a
            * (1.0 - ss::tax::trabalhador - irs::retencao::tax(self.cat_a / self.meses_cat_a, 0))
            + self.subsidio_alimentacao * self.meses_cat_a
    }

    fn liquido_cat_b(&self) -> Money {
        self.cat_b - self.ss_desc_cat_b()
    }

    fn liquido_total(&self) -> Money {
        self.liquido_cat_a() + self.liquido_cat_b()
    }

    fn liquido_total_com_subsidios(&self) -> Money {
        self.cat_a
            * (1.0 - ss::tax::trabalhador - irs::retencao::tax(self.cat_a / self.meses_cat_a, 0))
            * (14.0 / 12.0)
            + self.subsidio_alimentacao * self.meses_cat_a
            + self.liquido_cat_b()
    }

    fn irs_rend(&self) -> Money {
        self.cat_a * (14.0 / 12.0) + self.cat_b * 0.75
    }

    fn irs_retencao(&self) -> Money {
        self.cat_a * irs::retencao::tax(self.cat_a / self.meses_cat_a, 0) * (14.0 / 12.0)
    }

    fn irs_coleta_total(&self) -> Money {
        let rend_coletavel = self.irs_rend() - 4104.0;
        let (_, (_, tax, a_abater)) = irs::escalao(rend_coletavel);
        rend_coletavel * tax - a_abater
    }

    fn ss_rend_cat_a(&self) -> Money {
        self.cat_a * (14.0 / 12.0)
    }

    fn ss_rend_cat_b(&self) -> Money {
        self.cat_b * 0.70 * (1.0 + (self.cat_b_opt as isize) as f64 / 100.0)
    }

    fn ss_rend(&self) -> Money {
        self.ss_rend_cat_a() + self.ss_rend_cat_b()
    }

    fn ss_desc_cat_a_trab(&self) -> Money {
        self.cat_a * ss::tax::trabalhador * (14.0 / 12.0)
    }

    fn ss_desc_cat_a_empr(&self) -> Money {
        self.cat_a * ss::tax::empresa * (14.0 / 12.0)
    }

    fn ss_desc_cat_a_total(&self) -> Money {
        self.cat_a * ss::tax::cat_a * (14.0 / 12.0)
    }

    fn ss_desc_cat_b(&self) -> Money {
        self.cat_b * ss::tax::cat_b * (1.0 + (self.cat_b_opt as isize) as f64 / 100.0)
    }

    fn ss_desc_total(&self) -> Money {
        self.ss_desc_cat_a_total() + self.ss_desc_cat_b()
    }

    fn print_summary<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        let meses = if self.meses_cat_a > self.meses_cat_b {
            self.meses_cat_a
        } else {
            self.meses_cat_b
        };
        fn print<W: Write>(w: &mut W, txt: &str, t: f64, c: f64) -> std::io::Result<()> {
            writeln!(w, "{} (mês⁻¹ / total): {:.2} / {:.2}", txt, t / c, t)
        }
        writeln!(w, "Categoria B ({} meses):", self.meses_cat_b)?;
        print(w, "\tBruto", self.cat_b, self.meses_cat_b)?;
        print(w, "\tLimpo", self.liquido_cat_b(), self.meses_cat_b)?;
        writeln!(w, "Categoria A ({} meses):", self.meses_cat_a)?;
        print(w, "\tSalário bruto", self.cat_a, self.meses_cat_a)?;
        print(w, "\tSubsídios brutos", self.cat_a / 6.0, self.meses_cat_a)?;
        print(w, "\tSalário líq", self.liquido_cat_a(), self.meses_cat_a)?;
        print(
            w,
            "\tSubsídios líqs",
            self.liquido_cat_a() / 6.0,
            self.meses_cat_a,
        )?;
        print(w, "Total limpo", self.liquido_total(), meses)?;
        print(
            w,
            "Total limpo com subs.",
            self.liquido_total_com_subsidios(),
            meses,
        )?;
        print(
            w,
            "Total bruto.",
            self.cat_a * 14.0 / 12.0 + self.subsidio_alimentacao * self.meses_cat_a + self.cat_b,
            meses,
        )?;
        print(w, "SS rendiment", self.ss_rend(), meses)?;
        print(w, "SS descontos", self.ss_desc_total(), meses)?;
        writeln!(w, "IRS retencao feita: {:.2}", self.irs_retencao())?;
        writeln!(w, "IRS rendimento global: {:.2}", self.irs_rend())?;
        writeln!(
            w,
            "IRS coleta total - retenções: {:.2}",
            self.irs_coleta_total() - self.irs_retencao()
        )?;
        writeln!(
            w,
            "Rendimento total relevante: {:.2}",
            self.liquido_total_com_subsidios() + self.irs_retencao()
        )?;
        Ok(())
    }
}

fn distribuir(total: Money, meses_cat_a: f64, meses_cat_b: f64) {
    let alimentacao = irs::limites::subsidio_refeição * f64::from(DIAS_POR_MES);
    let print = |cat_b, cat_b_var| {
        Rendimentos::new(
            empresa2bruto((total - cat_b) / meses_cat_a, alimentacao) * meses_cat_a,
            meses_cat_a,
            cat_b,
            meses_cat_b,
            alimentacao,
            cat_b_var,
        )
        .unwrap()
        .print_summary(&mut std::io::stdout())
        .unwrap();
        println!();
    };

    print(
        irs::limites::isencao_retencao_cat_b - 3.33 * 7.0,
        SegSocVarCatB::P15,
    );
    /*print(
        (meses_cat_b / 3.0).ceil() * 3.0 * lim_isencao_retencao_na_fonte_irs_cat_b.floor(),
        SegSocVarCatB::Zero,
    );
    print(
        meses_cat_b * lim_isencao_retencao_na_fonte_irs_cat_b.floor(),
        SegSocVarCatB::Zero,
    );*/
    print(0.0, SegSocVarCatB::Zero);
}

fn main() {
    /*let disponivel = 2353.0 * 1.23 * 7.0;
    println!(
        "Total disponivel (mês⁻¹ / total): {:.2} / {:.2}",
        disponivel / 7.0,
        disponivel
    );*/
    productivity(0);
    //distribuir(disponivel, 6.0, 7.0);

    let bruto = liquido2bruto(1250.0, 0.0, 0).unwrap()[0];
    println!(
        "350€+100€ renda:\n\tBruto {}\tLíquido {}\tRetenção {}\tCusto empresa {}",
        bruto,
        1250.0 - 100.0,
        bruto * irs::retencao::tax(bruto, 0),
        bruto2empresa(bruto, 0.0)
    );
    let bruto = liquido2bruto(1150.0, 0.0, 0).unwrap()[0];
    println!(
        "450€+  0€ renda:\n\tBruto {}\tLíquido {}\tRetenção {}\tCusto empresa {}",
        bruto,
        1150.0 - 0.0,
        bruto * irs::retencao::tax(bruto, 0),
        bruto2empresa(bruto, 0.0) + 100.0
    );
    for bruto in liquido2bruto(605.0, 0.0, 0).unwrap().into_iter() {
        println!(
            "Bruto {}\tRetenção {}\tCusto empresa {}",
            bruto,
            bruto * irs::retencao::tax(bruto, 0),
            bruto2empresa(bruto, 0.0)
        );
    }
    for bruto in liquido2bruto(606.0, 0.0, 0).unwrap().into_iter() {
        println!(
            "Bruto {}\tRetenção {}\tCusto empresa {}",
            bruto,
            bruto * irs::retencao::tax(bruto, 0),
            bruto2empresa(bruto, 0.0)
        );
    }
}

fn productivity(dependentes: usize) {
    let desired_incomes = [756.21, 1000.0, 1600.0, 2000.0];
    let useful_days = 365.0 - 13.0 - 22.0 - 52.17857 * 2.0;

    let food_allowance_monthly = (irs::limites::subsidio_refeição * useful_days) / 12.0;
    println!(" Income  \t   Expense   \tMin day prod\t   Expense   \tMin day prod\t");
    println!(
        "(€ pax⁻¹)\t  (€ pax⁻¹)  \t  (€ pax⁻¹) \t     (€)     \t     (€)    \t"
    );
    println!(
        "------------------------------------------------------------------------------------"
    );
    for &desired_income in desired_incomes.iter() {
        let bruto = liquido2bruto(desired_income, food_allowance_monthly, dependentes).unwrap()[0];
        let monthly_expense = bruto2empresa(bruto, food_allowance_monthly);
        let minimum_daily_productivity = monthly_expense * 14.0 / useful_days;
        println!(
            " {:7.2}  \t   {:7.2}   \t   {:7.2}  \t   {:7.2}   \t   {:7.2}  \t",
            desired_income,
            monthly_expense,
            minimum_daily_productivity,
            monthly_expense * 2.0,
            minimum_daily_productivity * 2.0
        );
        /*
        desired_income -= food_allowance_monthly
        if desired_income < minimum_wage:
            print(" N/A ")
        else:
            monthly_expense = desired_income/(1.0-tax_irs-tax_ssw)/(1.0-tax_ssc)*14/12 + food_allowance_monthly
            minimum_daily_productivity = (monthly_expense * 12 + food_allowance) / useful_days
            print(" {:7.2f}  \t   {:7.2f}   \t   {:7.2f}  \t   {:7.2f}   \t   {:7.2f}  \t".format(desired_income, monthly_expense, minimum_daily_productivity, monthly_expense*2, minimum_daily_productivity*2))
        */
    }
}
