use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use anyhow::{bail, Result};

use super::{Money, MoneyRate};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct QuantityPerTime<T, P> {
	pub qty: T,
	period: P,
}
impl<T, P> QuantityPerTime<T, P> {
	pub const fn new(quantity: T, period: P) -> Self { Self { qty: quantity, period } }

	pub const fn quantity_ref(&self) -> &T { &self.qty }

	pub const fn period_ref(&self) -> &P { &self.period }
}

impl<T: Copy + Sized, P> QuantityPerTime<T, P> {
	pub fn quantity(&self) -> T { self.qty }
}

impl<T, P: Copy + Sized> QuantityPerTime<T, P> {
	pub fn period(&self) -> P { self.period }
}

impl<T: Clone + Add<T, Output = T>, P1, P2> Add<QuantityPerTime<T, P2>> for QuantityPerTime<T, P1>
where
	QuantityPerTime<T, Yearly>: From<QuantityPerTime<T, P2>>,
	QuantityPerTime<T, Yearly>: From<QuantityPerTime<T, P1>>,
{
	type Output = QuantityPerTime<T, Yearly>;

	fn add(self, rhs: QuantityPerTime<T, P2>) -> Self::Output {
		QuantityPerTime {
			qty: QuantityPerTime::<T, Yearly>::from(self)
				.qty
				.add(QuantityPerTime::<T, Yearly>::from(rhs).qty),
			period: Yearly,
		}
	}
}

impl<T: Clone + Add<T, Output = T>, P: Clone + Eq> QuantityPerTime<T, P> {
	pub fn add(&self, rhs: &Self) -> Result<Self> {
		if self.period == rhs.period {
			Ok(Self { qty: self.qty.clone() + rhs.qty.clone(), period: self.period.clone() })
		} else {
			bail!("Period must be the same")
		}
	}
}
impl<T: PartialOrd, P: Eq> PartialOrd for QuantityPerTime<T, P> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		if self.period == other.period {
			self.qty.partial_cmp(&other.qty)
		} else {
			None
		}
	}
}

impl<T: Neg<Output = T>, P> Neg for QuantityPerTime<T, P> {
	type Output = Self;

	fn neg(self) -> Self::Output { QuantityPerTime { qty: self.qty.neg(), period: self.period } }
}

impl<U, T: Mul<U, Output = T>, P> Mul<U> for QuantityPerTime<T, P> {
	type Output = Self;

	fn mul(self, rhs: U) -> Self::Output { QuantityPerTime { qty: self.qty.mul(rhs), period: self.period } }
}

impl<U, T: MulAssign<U>, P> MulAssign<U> for QuantityPerTime<T, P> {
	fn mul_assign(&mut self, rhs: U) { self.qty.mul_assign(rhs) }
}

impl<U, T: Div<U, Output = T>, P> Div<U> for QuantityPerTime<T, P> {
	type Output = Self;

	fn div(self, rhs: U) -> Self::Output { QuantityPerTime { qty: self.qty.div(rhs), period: self.period } }
}

impl<U, T: DivAssign<U>, P> DivAssign<U> for QuantityPerTime<T, P> {
	fn div_assign(&mut self, rhs: U) { self.qty.div_assign(rhs) }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Yearly;
impl MoneyRate<Yearly> {
	pub const fn zero() -> Self { Self::new(Money::new(0.0), Yearly) }
}
impl Default for MoneyRate<Yearly> {
	fn default() -> Self { Self::zero() }
}
impl<T: AddAssign<T>> AddAssign<Self> for QuantityPerTime<T, Yearly> {
	fn add_assign(&mut self, rhs: Self) { self.qty.add_assign(rhs.qty) }
}
impl<T: Sub<T, Output = T>> Sub<Self> for QuantityPerTime<T, Yearly> {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		QuantityPerTime { qty: self.qty.sub(rhs.qty), period: self.period }
	}
}
impl<T: SubAssign<T>> SubAssign<Self> for QuantityPerTime<T, Yearly> {
	fn sub_assign(&mut self, rhs: Self) { self.qty.sub_assign(rhs.qty) }
}
impl<T: Mul<f64, Output = T>> From<QuantityPerTime<T, Monthly>> for QuantityPerTime<T, Yearly> {
	fn from(monthly: QuantityPerTime<T, Monthly>) -> Self {
		Self { qty: monthly.qty * monthly.period.months_in_year(), period: Yearly }
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Monthly {
	M11,
	M12,
	M14,
}

impl Monthly {
	fn months_in_year(&self) -> f64 {
		match self {
			Self::M11 => 11.0,
			Self::M12 => 12.0,
			Self::M14 => 14.0,
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Workdaily;
impl Workdaily {
	pub fn actual_workdays_in_year(year: u32, vacation_days: u32) -> u32 {
		match year {
			2022 => 251 - vacation_days, // https://www.dias-uteis.pt/dias-uteis_feriados_2022.htm
			_ => unimplemented!(),
		}
	}
}

impl<T: Mul<f64, Output = T>> QuantityPerTime<T, Workdaily> {
	pub fn into_yearly(self, workdays_in_year: Option<u32>) -> QuantityPerTime<T, Yearly> {
		let workdays_in_year = match workdays_in_year {
			Some(days) => days as f64,
			None => 11.0 * 22.0,
		};
		QuantityPerTime { qty: self.qty * workdays_in_year, period: Yearly }
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Hourly {
	hours_per_week: f64,
}

impl<T: Mul<f64, Output = T>> From<QuantityPerTime<T, Hourly>> for QuantityPerTime<T, Yearly> {
	fn from(hourly: QuantityPerTime<T, Hourly>) -> Self {
		Self { qty: hourly.qty * hourly.period.hours_per_week * 52.0, period: Yearly }
	}
}
