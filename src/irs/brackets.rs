use crate::units::{Money, MoneyRate, TaxRate, Yearly};

fn positive_difference(a: f64, b: f64) -> f64 { (a - b).max(0.0) }

pub fn taxes(income: MoneyRate<Yearly>, brackets: &[Bracket]) -> MoneyRate<Yearly> {
	let value = positive_difference(income.quantity().value(), 4104.0);
	let mut taxed = 0.0.into();
	let mut prev_bracket_value = 0.0;
	for bracket in brackets.iter() {
		if bracket.rate.quantity().value() > value {
			taxed += (bracket.tax * (value - prev_bracket_value)).into();
			break;
		} else {
			taxed += (bracket.tax * (bracket.rate.quantity().value() - prev_bracket_value)).into();
		}
		prev_bracket_value = bracket.rate.quantity().value();
	}
	MoneyRate::new(taxed, Yearly)
}

pub struct Bracket {
	rate: MoneyRate<Yearly>,
	tax: TaxRate,
}

impl Bracket {
	const fn new(yearly: f64, tax: TaxRate) -> Self {
		Self { rate: MoneyRate::new(Money::new(yearly), Yearly), tax }
	}

	pub fn rate(self) -> MoneyRate<Yearly> { self.rate }

	pub fn tax(self) -> TaxRate { self.tax }
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

pub fn escalao(rendimento_colectavel: f64) -> (usize, (MoneyRate<Yearly>, f64, f64)) {
	(year_2022.iter())
		.scan((0.0, 0.0, 0.001), |(lim, tax, a_abater), Bracket { rate: new_lim, tax: new_tax }| {
			if rendimento_colectavel < *lim {
				//println!("{}º escalao (<{:.2}): {:.2} * {:.2}% - {:.2}", i + 1, lim,
				// rend_coletavel, tax * 100.0, a_abater);
				return None;
			}
			*a_abater += *lim * new_tax - *lim * *tax;
			*lim = new_lim.quantity().value();
			*tax = *new_tax;
			Some((*new_lim, *new_tax, *a_abater))
		})
		.enumerate()
		.last()
		.unwrap()
}
