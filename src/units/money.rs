use std::{
	fmt::Display,
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Money(f64);
impl Money {
	pub const fn new(value: f64) -> Self { Self(value) }

	pub const fn value(&self) -> f64 { self.0 }

	pub fn abs(&self) -> Self { Money(self.0.abs()) }
}

impl From<f64> for Money {
	fn from(money: f64) -> Self { Money(money) }
}

impl Display for Money {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let precision = if let Some(p) = f.precision() { p } else { 2 };
		if let Some(width) = f.width() {
			write!(f, "{:width$.precision$}€", self.0, width = width - 1 /* for € */)
		} else {
			write!(f, "{:.precision$}€", self.0)
		}
	}
}

impl Mul<f64> for Money {
	type Output = Self;

	fn mul(self, rhs: f64) -> Self::Output { Money(self.0.mul(rhs)) }
}
impl Mul<Money> for f64 {
	type Output = Money;

	fn mul(self, rhs: Self::Output) -> Self::Output { Money(self.mul(rhs.0)) }
}
impl MulAssign<f64> for Money {
	fn mul_assign(&mut self, rhs: f64) { self.0.mul_assign(rhs) }
}
impl Div<f64> for Money {
	type Output = Self;

	fn div(self, rhs: f64) -> Self::Output { Money(self.0.div(rhs)) }
}
impl DivAssign<f64> for Money {
	fn div_assign(&mut self, rhs: f64) { self.0.div_assign(rhs) }
}
impl Add<Self> for Money {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output { Money(self.0.add(rhs.0)) }
}
impl AddAssign<Self> for Money {
	fn add_assign(&mut self, rhs: Self) { self.0.add_assign(rhs.0) }
}
impl Sub<Self> for Money {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output { Money(self.0.sub(rhs.0)) }
}
impl SubAssign<Self> for Money {
	fn sub_assign(&mut self, rhs: Self) { self.0.sub_assign(rhs.0) }
}
