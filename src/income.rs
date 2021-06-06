use crate::irs::{self, Taxing as IrsTaxing};
use crate::{ss, ElementoFamiliar, Frequency, Money, MoneyRate, Tax};

pub trait SalaryHeading: core::fmt::Debug + SalaryHeadingBoxClone {
	fn payment(&self) -> MoneyRate;
	fn ss_taxing(&self) -> bool;
	fn irs_taxing(&self) -> IrsTaxing;
	fn company_cost(&self) -> MoneyRate {
		let mut payment = self.payment().as_yearly();
		if self.ss_taxing() {
			payment.value *= 1.0 + ss::tax::empresa;
		}
		payment
	}

	fn net(&self, family: &ElementoFamiliar) -> MoneyRate {
		let mut payment = self.payment();
		let mut taxes = 0.0;
		if self.irs_taxing() == IrsTaxing::TaxedWitheld {
			taxes += irs::withholding::year_2022::tax(payment, family).unwrap();
		}
		if self.ss_taxing() {
			taxes += ss::tax::trabalhador;
		}
		payment.value *= 1.0 - taxes;
		payment
	}
}

// Splitting SalaryHeadingBoxClone into its own trait allows us to provide a
// blanket implementation for all compatible types, without having to implement
// the rest of Rendimento. In this case, we implement it for all types that
// have 'static lifetime (*i.e.* they don't contain non-'static pointers), and
// implement both Rendimento and Clone. Don't ask me how the compiler resolves
// implementing SalaryHeadingBoxClone for Rendimento when Rendimento requires
// SalaryHeadingBoxClone; I have **no idea** why that works.
pub trait SalaryHeadingBoxClone {
	fn clone_box(&self) -> Box<dyn SalaryHeading>;
}

impl<T> SalaryHeadingBoxClone for T
where
	T: 'static + SalaryHeading + Clone,
{
	fn clone_box(&self) -> Box<dyn SalaryHeading> { Box::new(self.clone()) }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn SalaryHeading> {
	fn clone(&self) -> Box<dyn SalaryHeading> { self.clone_box() }
}

/*
pub fn yearly_net_income(&self) -> MoneyRate {
	let mut payment = self.payment.as_yearly();
	if self.ss_taxed {
		payment.value *= 1.0 - ss::tax::trabalhador;
	}
	if self.irs_taxing != IrsTaxing::None {
		payment.value -= irs::brackets::tax_value(payment, &irs::brackets::ano_2021);
	}
	payment
}
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Deslocacao {
	InternacionalDirector,
	InternacionalOutros,
	NacionalDirector,
	NacionalOutros,
}

#[derive(Debug, Clone, Copy)]
pub struct BaseSalary {
	pub monthly: Money,
}

impl SalaryHeading for BaseSalary {
	fn payment(&self) -> MoneyRate { MoneyRate::new(self.monthly, Frequency::Monthly14) }

	fn ss_taxing(&self) -> bool { true }

	fn irs_taxing(&self) -> IrsTaxing { IrsTaxing::TaxedWitheld }
}

#[derive(Debug, Clone, Copy)]
pub struct MealAllowance {
	pub on_card: bool,
}

impl SalaryHeading for MealAllowance {
	fn payment(&self) -> MoneyRate {
		match self.on_card {
			true => crate::non_taxation_limits::vale_refeição,
			false => crate::non_taxation_limits::subsidio_refeição,
		}
	}

	fn ss_taxing(&self) -> bool { false }

	fn irs_taxing(&self) -> IrsTaxing { IrsTaxing::None }
}

#[derive(Debug, Clone, Copy)]
pub struct TravelExpenses {
	pub monthly: Money,
}

impl SalaryHeading for TravelExpenses {
	fn payment(&self) -> MoneyRate { MoneyRate::new(self.monthly, Frequency::Monthly11) }

	fn ss_taxing(&self) -> bool { false }

	fn irs_taxing(&self) -> IrsTaxing { IrsTaxing::None }
}

#[derive(Debug, Clone, Copy)]
pub struct RetirementFunds {
	pub monthly: Money,
}

impl SalaryHeading for RetirementFunds {
	fn payment(&self) -> MoneyRate { MoneyRate::new(self.monthly, Frequency::Monthly) }

	fn ss_taxing(&self) -> bool { false }

	fn irs_taxing(&self) -> IrsTaxing { IrsTaxing::Taxed }
}

#[derive(Debug, Clone)]
pub struct Income {
	base_salary: BaseSalary,
	meal_allowance: MealAllowance,
	travel_expenses: TravelExpenses,
	retirement_funds: RetirementFunds,
	insurance_tax: Tax,
	// TODO: incluir ValeInfancia (muito vantajoso para empresa e trabalhadores)

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

impl Income {
	pub fn new(base: Money, cost_aid: Money, retirement: Money, meal_card: bool, insurance_tax: Tax) -> Self {
		Income {
			base_salary: BaseSalary { monthly: base },
			meal_allowance: MealAllowance { on_card: meal_card },
			travel_expenses: TravelExpenses { monthly: cost_aid },
			retirement_funds: RetirementFunds { monthly: retirement },
			insurance_tax,
		}
	}

	pub fn company_cost(&self) -> MoneyRate {
		MoneyRate::new_yearly(
			self.base_salary.company_cost().as_yearly().value()
				+ self.meal_allowance.company_cost().as_yearly().value()
				+ self.travel_expenses.company_cost().as_yearly().value() * 1.05
				+ self.retirement_funds.company_cost().as_yearly().value()
				+ (self.base_salary.monthly * 14.679 + 1222.24) * self.insurance_tax * 1.34775
				+ 30.0 * 12.0, // medical insurance
		)
	}

	pub fn monthly_net_average(&self, family: &ElementoFamiliar) -> MoneyRate {
		MoneyRate::new(
			(self.base_salary.net(family).as_yearly().value()
				+ self.meal_allowance.net(family).as_yearly().value()
				+ self.travel_expenses.net(family).as_yearly().value()
				+ self.retirement_funds.net(family).as_yearly().value())
				/ 12.0,
			Frequency::Monthly,
		)
	}

	pub fn monthly_net_typical(&self, family: &ElementoFamiliar) -> MoneyRate {
		MoneyRate::new(
			self.base_salary.net(family).value()
				+ self.meal_allowance.net(family).value() * 22.0
				+ self.travel_expenses.net(family).value()
				+ self.retirement_funds.net(family).value(),
			Frequency::Monthly,
		)
	}

	pub fn annual_irs(&self) -> Money {
		irs::brackets::taxes(
			&[&self.base_salary, &self.meal_allowance, &self.travel_expenses, &self.retirement_funds],
			&irs::brackets::year_2022,
		)
	}

	pub fn irs_retirement_taxes(&self) -> Money {
		self.annual_irs()
			- irs::brackets::taxes(
				&[&self.base_salary, &self.meal_allowance, &self.travel_expenses],
				&irs::brackets::year_2022,
			)
	}

	pub fn print(&self, family: &ElementoFamiliar) {
		println!(
			" {:10.2}   ({:8.2})  {:8.2}  ({:7.2})   {:8.2}  ({:7.2})   {:6.2} ({:6.2})   {:4.2}   {:6.2}   \
			 {:6.2}  ({:6.2})   {:8.2}  {:6.2}  {:6.2}",
			self.company_cost().value(),
			self.company_cost().value() + self.irs_retirement_taxes(),
			self.monthly_net_average(family).value(),
			self.monthly_net_average(family).value() - self.irs_retirement_taxes() / 12.0,
			self.monthly_net_typical(family).value(),
			self.monthly_net_typical(family).value() - self.irs_retirement_taxes() / 12.0,
			self.base_salary.payment().value(),
			self.base_salary.net(family).value(),
			self.meal_allowance.payment().value(),
			self.travel_expenses.payment().value(),
			self.retirement_funds.payment().value(),
			self.retirement_funds.payment().value() + self.irs_retirement_taxes() / 12.0,
			self.irs_retirement_taxes(),
			self.monthly_net_typical(family).value() * 12.0 / self.company_cost().value() * 100.0,
			self.monthly_net_average(family).value() * 12.0 / self.company_cost().value() * 100.0,
		);
	}
}
