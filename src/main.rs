//"5842.77*0.75+(5842.77*0.1-230.20*12)" Inclusao CPAS rendimento global
//transição base incidência de 75% para 90%:
//rendimento global * 0.15 - (4104 + despesas afectas actividade) = X
//Se X > 0, rendimento global += X (i.e. o excesso é tributado a 90% em vez de
// 75%)

//Seguro de trabalho Programador "(salário mensal*14.679+1222.24)*0.0055(juros)*1.34775"

#![feature(const_fn_floating_point_arithmetic)]
#![feature(trait_alias)]
#![allow(non_upper_case_globals)]

pub mod irs;
pub mod salary;
pub mod ss;
pub mod units;

use salary::Salary;
use units::*;

pub mod non_taxation_limits {
	use crate::units::{Money, MoneyRate, Workdaily, Yearly};
	pub const SUBSIDIO_REFEICAO: MoneyRate<Workdaily> = MoneyRate::new(Money::new(4.77), Workdaily);
	pub const VALE_REFEICAO: MoneyRate<Workdaily> = MoneyRate::new(Money::new(7.63), Workdaily);
	pub const AJUDAS_CUSTO_KM: Money = Money::new(0.36);
	pub const AJUDAS_CUSTO_DIA: MoneyRate<Workdaily> = MoneyRate::new(Money::new(50.20), Workdaily);
	pub const ISENCAO_RETENCAO_CAT_B: MoneyRate<Yearly> = MoneyRate::new(Money::new(10_000.0), Yearly);
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FamilyElement {
	casado: bool,
	titular_unico: bool,
	dependentes: usize,
}

//mínimo de existência = 1.5 IAS * 14
#[allow(dead_code)]
const IAS: MoneyRate<Monthly> = MoneyRate::new(Money::new(443.20), Monthly::M14); //2022 438.81; // 2021 438.81; // 2020 435.76; // 2019
#[allow(dead_code)]
const SALARIO_MINIMO: MoneyRate<Monthly> = MoneyRate::new(Money::new(705.00), Monthly::M14); //2022 665.00; //2021 635.00; //2020 600.0; // 2019

fn main() {
	let salary_context = salary::ContextBuilder::default().build().unwrap();
	println!(
		" Company cost   Net Avg    Net Typ    Base       Meal      Aids      Retire.    Typ %    Avg %"
	);
	print(&Salary::new(1900.0.into(), 400.0.into(), 300.0.into(), Some(true)), &salary_context); // Jorge
	print(&Salary::new(1270.0.into(), 400.0.into(), 0.0.into(), Some(true)), &salary_context); // Ricardo
	print(&Salary::new(1865.0.into(), 450.0.into(), 600.0.into(), Some(true)), &salary_context); // José
	println!("--------------");
	print(&Salary::new(4060.0.into(), 600.0.into(), 1200.0.into(), Some(true)), &salary_context);
	// 50k net
}

fn print(salary: &Salary, ctx: &salary::Context) {
	use salary::Heading;
	let cost = salary.company_cost(ctx).quantity();
	let plan = salary.yearly_plan_withhold_net(ctx);
	println!(
		" {:10}    {:8}   {:8}   {:8}   {:7}   {:7}   {:8}   {:6.2}   {:6.2}",
		cost + (200.0 * 12.0 + 600.0).into(),
		plan.yearly_total().quantity() / 12.0,
		plan.regular,
		salary.base_salary.gross_payment().quantity(),
		salary.meal_allowance.gross_payment().quantity(),
		salary.travel_expenses.gross_payment().quantity(),
		salary.retirement_funds.gross_payment().quantity(),
		plan.regular.value() / cost.value() * 12.0 * 100.0,
		plan.yearly_total().quantity().value() / cost.value() * 100.0,
	);
}
