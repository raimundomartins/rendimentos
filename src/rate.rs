use std::ops::Mul;

pub type Money = f64;
pub type MoneyRate = Rate<Money>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Frequency {
	Yearly,
	Monthly,
	Monthly11,
	Monthly14,
	Hourly,
	Workdaily,
}

impl Frequency {
	pub const fn ratio_to_yearly(self) -> f64 {
		match self {
			Frequency::Yearly => 1.0,
			Frequency::Monthly => 12.0,
			Frequency::Monthly11 => 11.0,
			Frequency::Monthly14 => 14.0,
			Frequency::Hourly => 40.0 * 52.0,
			Frequency::Workdaily => (251.0 - 22.0), /* 2022 https://www.dias-uteis.pt/dias-uteis_feriados_2022.htm */
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Rate<T> {
	pub value: T,
	pub freq: Frequency,
}

impl<T> Rate<T> {
	pub fn value_ref(&self) -> &T { &self.value }

	pub const fn new(value: T, freq: Frequency) -> Self { Self { value, freq } }

	pub const fn new_yearly(value: T) -> Self { Self { value, freq: Frequency::Yearly } }
}

impl<T: Copy> Rate<T> {
	pub fn value(&self) -> T { self.value }
}

impl<T: Clone + PartialEq + Mul<f64, Output = T>> PartialEq for Rate<T> {
	fn eq(&self, other: &Self) -> bool {
		if self.freq == other.freq {
			self.value_ref() == other.value_ref()
		} else {
			self.as_yearly().value_ref() == other.as_yearly().value_ref()
		}
	}
}

impl<T: Clone + PartialOrd + Mul<f64, Output = T>> PartialOrd for Rate<T> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		if self.freq == other.freq {
			self.value.partial_cmp(&other.value_ref())
		} else {
			self.as_yearly().value_ref().partial_cmp(&other.as_yearly().value_ref())
		}
	}
}

impl<T: Clone + Mul<f64, Output = T>> Rate<T> {
	pub fn as_yearly(&self) -> Self {
		Self { value: self.value.clone() * self.freq.ratio_to_yearly(), freq: Frequency::Yearly }
	}

	pub fn change_frequency(&mut self, freq: Frequency) -> &mut Self {
		if self.freq == freq {
			return self;
		}
		self.value = self.value.clone() * (self.freq.ratio_to_yearly() / freq.ratio_to_yearly());
		self.freq = freq;
		self
	}
}
