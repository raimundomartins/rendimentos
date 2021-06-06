//"5842.77*0.75+(5842.77*0.1-230.20*12)" Inclusao CPAS rendimento global
//transição base incidência de 75% para 90%:
//rendimento global * 0.15 - (4104 + despesas afectas actividade) = X
//Se X > 0, rendimento global += X (i.e. o excesso é tributado a 90% em vez de
// 75%)

//Seguro de trabalho Programador "(salário mensal*14.679+1222.24)*0.0055(juros)*1.34775"

#![feature(const_fn_floating_point_arithmetic)]
#![feature(trait_alias)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use anyhow::{ensure, Result};

pub mod income;
pub mod irs;
pub mod rate;
pub mod ss;

pub type Tax = f64;
use income::Income;
use rate::*;

pub mod non_taxation_limits {
	use crate::rate::{Frequency, Money, MoneyRate};
	pub const subsidio_refeição: MoneyRate = MoneyRate::new(4.77, Frequency::Workdaily);
	pub const vale_refeição: MoneyRate = MoneyRate::new(7.63, Frequency::Workdaily);
	pub const ajudas_custo_km: Money = 0.36;
	pub const ajudas_custo_dia: MoneyRate = MoneyRate::new(50.20, Frequency::Workdaily);
	pub const isencao_retencao_cat_b: MoneyRate = MoneyRate::new(10_000.0, Frequency::Yearly);
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ElementoFamiliar {
	casado: bool,
	titular_unico: bool,
	dependentes: usize,
}

const IAS: MoneyRate = MoneyRate::new(443.20, Frequency::Monthly); //438.81; // 2021 438.81; // 2020 435.76; // 2019
const salario_minimo: MoneyRate = MoneyRate::new(705.00, Frequency::Monthly); //2022 665.00; //2021 635.00; //2020 600.0; // 2019

fn bruto2liquido(bruto: MoneyRate, family: ElementoFamiliar) -> Money {
	bruto.value() * (1.0 - ss::tax::trabalhador - irs::withholding::year_2022::tax(bruto, &family).unwrap())
}

fn bruto2empresa(bruto: Money, subsidio_alimentacao_mensal: Money) -> Money {
	bruto * (1.0 + ss::tax::empresa) * 14.0 / 12.0 + subsidio_alimentacao_mensal
}

fn empresa2bruto(disponivel: Money, subsidio_alimentacao_mensal: Money) -> Money {
	(disponivel - subsidio_alimentacao_mensal) / (1.0 + ss::tax::empresa) / 14.0 * 12.0
}

fn main() {
	let family = ElementoFamiliar { casado: false, titular_unico: false, dependentes: 0 };
	let insurance_tax = 0.0055;
	println!(
		" Company cost (tax adj.)   Net Avg  (tax adj)   Net Typ   (tax adj)     Base    (Net)     Meal    \
		 Aids    Retire. (tax adj)   Ret IRS   Typ %   Avg %"
	);
	println!("-------------------------------------------------------------------------------------------------------------------------------------------------------");
	Income::new(2220.0, 200.0, 500.0, true, insurance_tax).print(&family);
	Income::new(1980.0, 200.0, 650.0, true, insurance_tax).print(&family);
	Income::new(1980.0, 400.0, 400.0, true, insurance_tax).print(&family);
	Income::new(1980.0, 400.0, 550.0, true, insurance_tax).print(&family);
	Income::new(1900.0, 400.0, 450.0, true, insurance_tax).print(&family);
	Income::new(1900.0, 400.0, 600.0, true, insurance_tax).print(&family);
	Income::new(2100.0, 200.0, 550.0, true, insurance_tax).print(&family);
	println!("-------------------------------------------------------------------------------------------------------------------------------------------------------");
	let family = ElementoFamiliar { casado: true, titular_unico: false, dependentes: 2 };
	Income::new(45000.0 / 14.0, 0.0, 0.0, true, insurance_tax).print(&family);
}

fn liquido2bruto(mut liquido: Money, alimentacao: Money, family: &ElementoFamiliar) -> Result<Vec<Money>> {
	liquido -= alimentacao;
	let bruto = |tax_irs| liquido / (1.0 - ss::tax::trabalhador - tax_irs);
	let mut result = vec![];
	let mut old_l0 = 0.0;
	let dependentes = family.dependentes;
	for l in irs::withholding::year_2022::tables.for_family(family).iter() {
		let b = bruto(l.1[std::cmp::min(5, dependentes)] / 100.0);
		//if b <= old_l0 { break; }
		if l.0 >= b && b >= salario_minimo.value() && b > old_l0 {
			result.push(b);
		}
		old_l0 = l.0;
	}
	ensure!(!result.is_empty(), "No match for {}€", liquido);
	Ok(result)
}
