use super::Taxing;
use crate::{income::SalaryHeading, Frequency, Money, MoneyRate, Tax};

//mínimo de existência = 1.5 IAS * 14
fn positive_difference(a: Money, b: Money) -> Money { (a - b).max(0.0) }

pub fn taxes(income: &[&dyn SalaryHeading], brackets: &[Bracket]) -> Money {
	let value = positive_difference(
		income
			.iter()
			.map(|s| if s.irs_taxing() != Taxing::None { s.payment().as_yearly().value() } else { 0.0 })
			.sum::<f64>(),
		4104.0,
	);
	let mut taxed = 0.0;
	let mut prev_bracket_value = 0.0;
	for bracket in brackets.iter() {
		if bracket.rate.value() > value {
			taxed += bracket.tax * (value - prev_bracket_value);
			break;
		} else {
			taxed += bracket.tax * (bracket.rate.value() - prev_bracket_value);
		}
		prev_bracket_value = bracket.rate.value();
	}
	taxed
}

pub struct Bracket {
	rate: MoneyRate,
	tax: Tax,
}

impl Bracket {
	const fn new(yearly: Money, tax: Tax) -> Self {
		Self { rate: MoneyRate { value: yearly, freq: Frequency::Yearly }, tax }
	}

	pub fn rate(self) -> MoneyRate { self.rate }

	pub fn tax(self) -> Tax { self.tax }
}

pub const year_2022: [Bracket; 9] = [
	Bracket::new(7_116.00, 0.145),
	Bracket::new(10_736.00, 0.230),
	Bracket::new(15_216.00, 0.265),
	Bracket::new(19_696.00, 0.285),
	Bracket::new(25_076.00, 0.350),
	Bracket::new(36_757.00, 0.370),
	Bracket::new(48_033.00, 0.435),
	Bracket::new(75_009.00, 0.450),
	Bracket::new(std::f64::INFINITY, 0.48),
];

pub const year_2021: [Bracket; 7] = [
	Bracket::new(7_112.00, 0.145),
	Bracket::new(10_732.00, 0.230),
	Bracket::new(20_322.00, 0.285),
	Bracket::new(25_075.00, 0.350),
	Bracket::new(36_967.00, 0.370),
	Bracket::new(80_882.00, 0.450),
	Bracket::new(std::f64::INFINITY, 0.48),
];

pub const year_2019: [Bracket; 7] = [
	Bracket::new(7_091.00, 0.145),
	Bracket::new(10_700.00, 0.230),
	Bracket::new(20_261.00, 0.285),
	Bracket::new(25_000.00, 0.350),
	Bracket::new(36_856.00, 0.370),
	Bracket::new(80_640.00, 0.450),
	Bracket::new(std::f64::INFINITY, 0.48),
];

pub fn escalao(rendimento_colectavel: f64) -> (usize, (MoneyRate, f64, f64)) {
	year_2021
		.iter()
		.scan((0.0, 0.0, 0.001), |(lim, tax, a_abater), Bracket { rate: new_lim, tax: new_tax }| {
			if rendimento_colectavel < *lim {
				//println!("{}º escalao (<{:.2}): {:.2} * {:.2}% - {:.2}", i + 1, lim,
				// rend_coletavel, tax * 100.0, a_abater);
				return None;
			}
			*a_abater += *lim * new_tax - *lim * *tax;
			*lim = new_lim.value();
			*tax = *new_tax;
			Some((*new_lim, *new_tax, *a_abater))
		})
		.enumerate()
		.last()
		.unwrap()
}
