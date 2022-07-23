use crate::{
	salary::{Heading as SalaryHeading, Salary},
	units::{Money, MoneyRate, TaxRate, Yearly},
};

pub struct Context {
	pub inem_tax: TaxRate,   // 0.02
	pub fat_tax: TaxRate,    // 0.0015
	pub stamp_duty: TaxRate, // 0.04
}
impl Default for Context {
	fn default() -> Self { Self { inem_tax: 0.02, fat_tax: 0.0015, stamp_duty: 0.04 } }
}

pub struct GeneralPolicy {
	pub premium_rate: TaxRate,
	pub record_cost: Money,
}

impl GeneralPolicy {
	pub fn new<M: Into<Money>>(premium_rate: TaxRate, record_cost: M) -> Self {
		Self { premium_rate, record_cost: record_cost.into() }
	}

	pub fn record_cost(&self, ctx: &Context) -> Money {
		self.record_cost * (1.0 + ctx.stamp_duty + ctx.inem_tax)
	}

	pub fn coverage_capital(&self, salary: &Salary) -> MoneyRate<Yearly> {
		salary.base_salary.gross_payment() + salary.meal_allowance.gross_payment()
	}

	pub fn premium(&self, salary: &Salary) -> MoneyRate<Yearly> {
		self.coverage_capital(salary) * self.premium_rate
	}

	pub fn fat_cost(&self, salary: &Salary, ctx: &Context) -> MoneyRate<Yearly> {
		self.coverage_capital(salary) * ctx.fat_tax
	}

	pub fn inem_cost(&self, salary: &Salary, ctx: &Context) -> MoneyRate<Yearly> {
		self.premium(salary) * ctx.inem_tax
	}

	pub fn total_cost(&self, salary: &Salary, ctx: &Context) -> MoneyRate<Yearly> {
		let capital = self.coverage_capital(salary);
		capital * (self.premium_rate + ctx.fat_tax) * (1.0 + ctx.inem_tax + ctx.stamp_duty)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn assert_eq(yearly: MoneyRate<Yearly>, expected: f64) {
		//println!("Yearly: {:?}\texpected: {:.02}", yearly, expected);
		assert!((yearly.quantity() - expected.into()).abs() < 0.0049.into())
	}

	#[test]
	fn creation_cost() {
		let ctx = Context { inem_tax: 0.02, fat_tax: 0.0015, stamp_duty: 0.04 };
		let ins = GeneralPolicy::new(0.0055, 5.0);
		assert!((ins.record_cost(&ctx).value() - 5.30).abs() < 0.0049);
	}

	#[test]
	fn first_three_workers() {
		let ctx = Context { inem_tax: 0.02, fat_tax: 0.0015, stamp_duty: 0.04 };
		let policy = GeneralPolicy::new(0.0055, 5.0);
		let salaries = vec![
			Salary::new(800.0, Some(false), 0.0),
			Salary::new(800.0, Some(false), 0.0),
			Salary::new(8.85 * 52.0 * 20.0 / 12.0, None, 0.0),
		];
		assert_eq(
			salaries.iter().fold(MoneyRate::zero(), |acc, sal| acc + policy.coverage_capital(sal)),
			35446.68,
		);
		let total_cost =
			salaries.iter().fold(MoneyRate::zero(), |acc, sal| acc + policy.total_cost(sal, &ctx));
		assert_eq(total_cost, 65.69 * 4.0 + 0.25); // 0.25€ unaccounted for
		assert_eq(
			MoneyRate::new(policy.record_cost(&ctx), Yearly) + total_cost,
			71.06 + 65.69 * 3.0 + 0.18, // 0.18€ unaccounted for
		);
	}

	#[test]
	fn four_workers() {
		let ctx = Context { inem_tax: 0.02, fat_tax: 0.0015, stamp_duty: 0.04 };
		let pol = GeneralPolicy::new(0.0055, 5.0);
		let salaries = vec![
			Salary::new(800.0, Some(false), 0.0),
			Salary::new(800.0, Some(false), 0.0),
			Salary::new(1270.0, Some(true), 0.0),
			Salary::new(2364.0, Some(true), 0.0),
		];
		assert_eq(
			salaries.iter().fold(MoneyRate::zero(), |acc, sal| acc + pol.coverage_capital(sal)),
			79277.60,
		);
		assert_eq(
			salaries.iter().fold(MoneyRate::zero(), |acc, sal| acc + pol.total_cost(sal, &ctx)),
			588.24,
		);
	}
}
