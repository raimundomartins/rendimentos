use derive_builder::Builder;

use crate::{ss, units::YearlyPlan, FamilyElement, Money, MoneyRate, Monthly, TaxRate, Yearly};

#[derive(Builder, Debug, Default, Clone, PartialEq)]
pub struct Context {
	#[builder(default = "2022")]
	pub year: u32,
	#[builder(default = "FamilyElement { casado: false, titular_unico: false, dependentes: 0 }")]
	pub family: FamilyElement,
	#[builder(default = "22")]
	pub vacation_days: u32,
	#[builder(default = "MoneyRate::<Yearly>::new(4.0, Yearly)")]
	pub meal_card_cost: MoneyRate<Yearly>,
	#[builder(default = "0.0078")]
	pub meal_card_tax: TaxRate,
	#[builder(default = "0.01")]
	pub salary_guarantee_fund_tax: TaxRate,
	#[builder(default = "ss::tax::EMPRESA")]
	pub company_tsu: TaxRate,
	#[builder(default = "ss::tax::TRABALHADOR")]
	pub worker_tsu: TaxRate,
	#[builder(default = "false")]
	pub impute_travel_expenses: bool, // to client
	#[builder(default = "0.05")]
	pub unimputed_travel_expenses_tax: TaxRate,
	#[builder(default = "0.0055")]
	pub work_insurance_tax: TaxRate,
	#[builder(default = "0.0015")]
	pub work_accident_fund_tax: TaxRate,
	#[builder(default = "0.04")]
	pub insurance_stamp_tax: TaxRate,
	#[builder(default = "0.025")]
	pub insurance_inem_tax: TaxRate,
}

#[derive(Builder, Debug, Default, Clone, PartialEq)]
pub struct StateTaxes {
	#[builder(default = "0.01")]
	pub salary_guarantee_fund_tax: TaxRate,
	#[builder(default = "ss::tax::EMPRESA")]
	pub company_tsu: TaxRate,
	#[builder(default = "ss::tax::TRABALHADOR")]
	pub worker_tsu: TaxRate,
	#[builder(default = "0.05")]
	pub unimputed_travel_expenses_tax: TaxRate,
	#[builder(default = "0.0015")]
	pub work_accident_fund_tax: TaxRate,
	#[builder(default = "0.04")]
	pub insurance_stamp_tax: TaxRate,
	#[builder(default = "0.025")]
	pub insurance_inem_tax: TaxRate,
}

pub trait Heading: core::fmt::Debug + HeadingBoxClone {
	fn gross_payment(&self) -> MoneyRate<Monthly>;
	fn ss_taxable_parcel(&self) -> MoneyRate<Monthly>;
	fn irs_taxable_parcel(&self) -> MoneyRate<Monthly>;
	fn company_cost(&self, ctx: &Context) -> MoneyRate<Yearly>;
}

// Splitting SalaryHeadingBoxClone into its own trait allows us to provide a
// blanket implementation for all compatible types, without having to implement
// the rest of Rendimento. In this case, we implement it for all types that
// have 'static lifetime (*i.e.* they don't contain non-'static pointers), and
// implement both Rendimento and Clone. Don't ask me how the compiler resolves
// implementing SalaryHeadingBoxClone for Rendimento when Rendimento requires
// SalaryHeadingBoxClone; I have **no idea** why that works.
pub trait HeadingBoxClone {
	fn clone_box(&self) -> Box<dyn Heading>;
}

impl<T> HeadingBoxClone for T
where
	T: 'static + Heading + Clone,
{
	fn clone_box(&self) -> Box<dyn Heading> { Box::new(self.clone()) }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Heading> {
	fn clone(&self) -> Box<dyn Heading> { self.clone_box() }
}

#[derive(Debug, Clone, Copy)]
pub struct BaseSalary {
	monthly: MoneyRate<Monthly>,
}
impl BaseSalary {
	pub fn new<M: Into<Money>>(monthly: M) -> Self {
		Self { monthly: MoneyRate::new(monthly.into(), Monthly::M14) }
	}

	pub fn monthly(&self) -> MoneyRate<Monthly> { self.monthly }
}
impl Heading for BaseSalary {
	fn gross_payment(&self) -> MoneyRate<Monthly> { self.monthly }

	fn ss_taxable_parcel(&self) -> MoneyRate<Monthly> { self.gross_payment() }

	fn irs_taxable_parcel(&self) -> MoneyRate<Monthly> { self.gross_payment() }

	fn company_cost(&self, ctx: &Context) -> MoneyRate<Yearly> {
		let payment: MoneyRate<Yearly> = self.gross_payment().into();
		payment * (1.0 + ctx.company_tsu + ctx.salary_guarantee_fund_tax)
	}
}

#[derive(Debug, Clone, Copy)]
pub struct MealAllowance {
	pub on_card: Option<bool>,
}
impl Heading for MealAllowance {
	fn gross_payment(&self) -> MoneyRate<Monthly> {
		MoneyRate::new(
			match self.on_card {
				Some(true) => {
					crate::non_taxation_limits::VALE_REFEICAO
						.into_yearly(None)//Some(Workdaily::actual_workdays_in_year(2022, 22)))
						.quantity() / 11.0
				}
				Some(false) => {
					crate::non_taxation_limits::SUBSIDIO_REFEICAO
						.into_yearly(None)//Some(Workdaily::actual_workdays_in_year(2022, 22)))
						.quantity() / 11.0
				}
				None => 0.0.into(),
			},
			Monthly::M11,
		)
	}

	fn ss_taxable_parcel(&self) -> MoneyRate<Monthly> { self.irs_taxable_parcel() }

	fn irs_taxable_parcel(&self) -> MoneyRate<Monthly> { MoneyRate::new(0.0, Monthly::M11) }

	fn company_cost(&self, ctx: &Context) -> MoneyRate<Yearly> {
		let yearly_paid = MoneyRate::<Yearly>::from(self.gross_payment());
		let card_cost = if self.on_card == Some(true) {
			ctx.meal_card_cost + yearly_paid * ctx.meal_card_tax
		} else {
			MoneyRate::zero()
		};
		yearly_paid + card_cost
	}
}

#[derive(Debug, Clone, Copy)]
pub struct TravelExpenses {
	pub monthly: MoneyRate<Monthly>,
}

impl Heading for TravelExpenses {
	fn gross_payment(&self) -> MoneyRate<Monthly> { self.monthly }

	fn ss_taxable_parcel(&self) -> MoneyRate<Monthly> { self.irs_taxable_parcel() }

	fn irs_taxable_parcel(&self) -> MoneyRate<Monthly> { MoneyRate::new(0.0, Monthly::M11) }

	fn company_cost(&self, ctx: &Context) -> MoneyRate<Yearly> {
		let mut payment: MoneyRate<Yearly> = self.gross_payment().into();
		if !ctx.impute_travel_expenses {
			payment *= 1.0 + ctx.unimputed_travel_expenses_tax;
		}
		payment
	}
}

#[derive(Debug, Clone, Copy)]
pub struct RetirementFunds {
	pub monthly: MoneyRate<Monthly>,
}
impl RetirementFunds {
	pub fn new<M: Into<Money>>(monthly: M) -> Self {
		Self { monthly: MoneyRate::new(monthly.into(), Monthly::M12) }
	}
}

impl Heading for RetirementFunds {
	fn gross_payment(&self) -> MoneyRate<Monthly> { self.monthly }

	fn ss_taxable_parcel(&self) -> MoneyRate<Monthly> { MoneyRate::new(0.0, Monthly::M12) }

	fn irs_taxable_parcel(&self) -> MoneyRate<Monthly> { self.gross_payment() }

	fn company_cost(&self, _ctx: &Context) -> MoneyRate<Yearly> { (self.gross_payment() * 1.02).into() }
}

#[derive(Debug, Clone)]
pub struct Salary {
	pub base_salary: BaseSalary,
	pub meal_allowance: MealAllowance,
	pub travel_expenses: TravelExpenses,
	pub retirement_funds: RetirementFunds,
	// TODO: incluir ValeInfancia (isentos de IRS e SS, e descontam 140% no IRC)

	/* TODO: incluir Indemnização cessação de trabalho
	(isento até Remuneração média dos últimos 12 meses * anos trabalho)
	Tratando-se de gestor, administrador, gerente de pessoa coletiva, gerente público ou representante de
	estabelecimento estável de entidade não residente, os montantes recebidos pela cessação do vínculo laboral
	são sujeitos a tributação na sua totalidade, apenas na parte respeitante a essas mesmas funções.
	A parte respeitante a períodos em que estes tenham exercido funções como trabalhador por conta de outrem
	continuam a beneficiar da exclusão de tributação. Relativamente à Segurança Social, não constituem base de
	incidência a compensação por cessação do contrato de trabalho no caso de despedimento coletivo; por
	extinção do posto de trabalho, por inadaptação; por não concessão de aviso prévio; por caducidade; por
	resolução por parte do trabalhador; por cessação antes de findo o prazo convencional do contrato de
	trabalho a prazo.
	*/
}

impl Salary {
	pub fn new<M1: Into<Money>, M2: Into<Money>>(base: M1, meal_card: Option<bool>, cost_aid: M2) -> Self {
		Salary {
			base_salary: BaseSalary { monthly: MoneyRate::new(base, Monthly::M14) },
			meal_allowance: MealAllowance { on_card: meal_card },
			travel_expenses: TravelExpenses { monthly: MoneyRate::new(cost_aid.into(), Monthly::M11) },
			retirement_funds: RetirementFunds { monthly: MoneyRate::new(0.0, Monthly::M12) },
		}
	}

	pub fn work_accidents_insurance(&self, ctx: &Context) -> MoneyRate<Yearly> {
		let capital = self.base_salary.gross_payment() + self.meal_allowance.gross_payment();
		// Nota: criação/alteração de apólice conta como premium para efeitos de aplicação de taxas
		let premium = capital * ctx.work_insurance_tax;
		(premium + capital * ctx.work_accident_fund_tax) * (1.0 + ctx.insurance_stamp_tax)
			+ premium * ctx.insurance_inem_tax
	}

	pub fn company_cost(&self, ctx: &Context) -> MoneyRate<Yearly> {
		let c1 = self.base_salary.company_cost(ctx);
		let c2 = self.meal_allowance.company_cost(ctx);
		let c3 = self.travel_expenses.company_cost(ctx);
		let c4 = self.retirement_funds.company_cost(ctx);
		let c5 = self.work_accidents_insurance(ctx);
		c1 + c2 + c3 + c4 + c5
	}

	pub fn yearly_plan_withhold_net(&self, ctx: &Context) -> YearlyPlan<Money> {
		let headings = self.headings();
		let plan: YearlyPlan<_> = headings.iter().map(|h| h.gross_payment()).into();
		let ss_taxable_parcel: YearlyPlan<_> = headings.iter().map(|h| h.ss_taxable_parcel()).into();
		let irs_taxable_parcel: YearlyPlan<_> = headings.iter().map(|h| h.irs_taxable_parcel()).into();
		let irs_withholding = match ctx.year {
			2022 => |v| v * crate::irs::withholding::year_2022::tax(v, &ctx.family),
			_ => unimplemented!(),
		};
		let irs = irs_taxable_parcel.map(irs_withholding);
		plan - ss_taxable_parcel * ctx.worker_tsu - irs
	}

	pub fn yearly_plan_real_net(&self, ctx: &Context) -> MoneyRate<Yearly> {
		let headings = self.headings();
		let plan: YearlyPlan<_> = headings.iter().map(|h| h.gross_payment()).into();
		let ss_taxable_parcel: YearlyPlan<_> = headings.iter().map(|h| h.ss_taxable_parcel()).into();
		let irs_taxable_parcel: YearlyPlan<_> = headings.iter().map(|h| h.irs_taxable_parcel()).into();
		// TODO: Add context to brackets calculation!
		let irs_tax =
			crate::irs::brackets::taxes(irs_taxable_parcel.yearly_total(), &crate::irs::brackets::year_2022);
		(plan - ss_taxable_parcel * ctx.worker_tsu).yearly_total() - irs_tax
	}

	fn headings(&self) -> [&dyn Heading; 4] {
		[
			&self.base_salary as &dyn Heading,
			&self.meal_allowance as &dyn Heading,
			&self.travel_expenses as &dyn Heading,
			&self.retirement_funds as &dyn Heading,
		]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn assert_eq(yearly: MoneyRate<Yearly>, expected: f64) {
		assert!((yearly.quantity() - expected.into()).abs() < 0.001.into())
	}

	#[test]
	fn base_salary_company_cost() {
		let ctx = ContextBuilder::default().build().unwrap();
		assert_eq(BaseSalary::new(1000.0).company_cost(&ctx), 1000.0 * (1.2375 + 0.01) * 14.0);
	}

	#[test]
	fn meal_allowance_company_cost() {
		let ctx = ContextBuilder::default().build().unwrap();
		assert_eq(MealAllowance { on_card: None }.company_cost(&ctx), 0.0);
		assert_eq(MealAllowance { on_card: Some(false) }.company_cost(&ctx), 4.77 * 22.0 * 11.0);
		assert_eq(
			MealAllowance { on_card: Some(true) }.company_cost(&ctx),
			7.63 * 22.0 * 11.0 * 1.0078 + 4.0,
		);
	}

	#[test]
	fn travel_expenses_company_cost() {
		let ctx =
			ContextBuilder::default().company_tsu(0.2375).salary_guarantee_fund_tax(0.01).build().unwrap();
		let salary_cost = BaseSalary::new(1000.0).company_cost(&ctx);
		assert_eq(salary_cost, 1000.0 * (1.2375 + 0.01) * 14.0);
	}

	#[test]
	fn retirement_fund_company_cost() {
		let ctx = ContextBuilder::default().build().unwrap();
		assert_eq(RetirementFunds::new(1000.0).company_cost(&ctx), 1000.0 * 12.0 * 1.02);
	}
}
