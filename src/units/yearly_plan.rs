use std::ops::{Add, AddAssign, Mul, Sub};

use super::{Monthly, QuantityPerTime, Yearly};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct YearlyPlan<T> {
	pub regular: T,
	pub vacation: T,
	pub bonus: T,
}

impl<T> YearlyPlan<T> {
	pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> YearlyPlan<U> {
		YearlyPlan { regular: f(self.regular), vacation: f(self.vacation), bonus: f(self.bonus) }
	}

	pub fn combine<U, V, F: FnMut(T, U) -> V>(self, other: YearlyPlan<U>, mut f: F) -> YearlyPlan<V> {
		YearlyPlan {
			regular: f(self.regular, other.regular),
			vacation: f(self.vacation, other.vacation),
			bonus: f(self.bonus, other.bonus),
		}
	}
}

impl<T: Copy + Mul<f64, Output = T> + Add<T, Output = T>> YearlyPlan<T> {
	pub fn yearly_total(&self) -> QuantityPerTime<T, Yearly> {
		QuantityPerTime::new(self.regular * 11.0 + self.vacation + self.bonus * 2.0, Yearly)
	}
}

impl<T: Copy + Mul<f64, Output = T>> Mul<f64> for YearlyPlan<T> {
	type Output = Self;

	fn mul(self, rhs: f64) -> Self::Output { self.map(|q| q * rhs) }
}

impl<T: Add<T, Output = T>> Add<Self> for YearlyPlan<T> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output { self.combine(rhs, |s, r| s.add(r)) }
}

impl<T: Sub<T, Output = T>> Sub<Self> for YearlyPlan<T> {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output { self.combine(rhs, |s, r| s.sub(r)) }
}

impl<T: Copy + AddAssign<T>> AddAssign<QuantityPerTime<T, Monthly>> for YearlyPlan<T> {
	fn add_assign(&mut self, rhs: QuantityPerTime<T, Monthly>) {
		let qty = rhs.qty;
		match rhs.period() {
			Monthly::M12 => {
				self.regular += qty;
				self.vacation += qty;
			}
			Monthly::M11 => {
				self.regular += qty;
			}
			Monthly::M14 => {
				self.regular += qty;
				self.vacation += qty;
				self.bonus += qty;
			}
		}
	}
}

impl<T: Copy + Default + AddAssign<T>, I: Iterator<Item = QuantityPerTime<T, Monthly>>> From<I>
	for YearlyPlan<T>
{
	fn from(iter: I) -> Self {
		let mut res = Self::default();
		iter.for_each(|rate| res += rate);
		res
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::units::{Money, MoneyRate};

	#[test]
	fn yearly_plan_yearly_total() {
		assert!(
			YearlyPlan { regular: Money::from(1000.0), vacation: 20.0.into(), bonus: 3.0.into() }
				.yearly_total() == MoneyRate::new(11026.0.into(), Yearly)
		)
	}

	#[test]
	fn yearly_plan_addassign_monthly_regular() {
		let mut plan = YearlyPlan::default();
		plan += QuantityPerTime::new(1.0, Monthly::M12);
		assert!(plan.regular == 1.0);
		assert!(plan.vacation == 1.0);
		assert!(plan.bonus == 0.0);
	}

	#[test]
	fn yearly_plan_addassign_monthly_useful() {
		let mut plan = YearlyPlan::default();
		plan += QuantityPerTime::new(1.0, Monthly::M11);
		assert!(plan.regular == 1.0);
		assert!(plan.vacation == 0.0);
		assert!(plan.bonus == 0.0);
	}

	#[test]
	fn yearly_plan_addassign_monthly_bonus() {
		let mut plan = YearlyPlan::default();
		plan += QuantityPerTime::new(1.0, Monthly::M14);
		assert!(plan.regular == 1.0);
		assert!(plan.vacation == 1.0);
		assert!(plan.bonus == 1.0);
	}
}
