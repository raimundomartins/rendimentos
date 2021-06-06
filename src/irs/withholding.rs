use anyhow::{bail, Result};

use crate::{rate::Frequency, ElementoFamiliar, Money, MoneyRate, Tax};

// You can obtain these tables by applying tabler.sh to the
// table selection at www.economias.pt/docs/tabelas_irs_YYYY.pdf:

/*pub struct RetentionLine {
	income: Money,
	tax: [Tax; 6],
}*/

pub struct RetentionTables {
	nao_casado: &'static [(Money, [Tax; 6])],
	casado_titular_unico: &'static [(Money, [Tax; 6])],
	casado_dois_titulares: &'static [(Money, [Tax; 6])],
}

impl RetentionTables {
	pub fn for_family(&self, family: &ElementoFamiliar) -> &'static [(Money, [Tax; 6])] {
		if family.casado {
			if family.titular_unico {
				self.casado_titular_unico
			} else {
				self.casado_dois_titulares
			}
		} else {
			self.nao_casado
		}
	}

	pub fn tax(&self, gross: MoneyRate, family: &ElementoFamiliar) -> Result<Tax> {
		if gross.freq != Frequency::Monthly
			&& gross.freq != Frequency::Monthly11
			&& gross.freq != Frequency::Monthly14
		{
			bail!("Can only calculate withholding tax of incomes of Frequency::Monthly*");
		}
		let dependentes = family.dependentes;
		for l in self.for_family(family).iter() {
			if l.0 >= gross.value() {
				return Ok(l.1[std::cmp::min(5, dependentes)] / 100.0);
			}
		}
		unreachable!();
	}
}

pub mod year_2022 {
	use super::*;
	pub const tables: RetentionTables = RetentionTables {
		nao_casado: &nao_casado,
		casado_titular_unico: &casado_titular_unico,
		casado_dois_titulares: &casado_dois_titulares,
	};

	pub fn tax(gross: MoneyRate, family: &ElementoFamiliar) -> Result<Tax> { tables.tax(gross, family) }

	pub const nao_casado: [(Money, [Tax; 6]); 36] = [
		(710.00, [0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
		(720.00, [1.8, 0.2, 0.0, 0.0, 0.0, 0.0]),
		(740.00, [4.5, 0.6, 0.0, 0.0, 0.0, 0.0]),
		(754.00, [6.3, 0.8, 0.0, 0.0, 0.0, 0.0]),
		(814.00, [7.9, 4.5, 1.0, 0.0, 0.0, 0.0]),
		(922.00, [10.1, 6.7, 3.5, 0.0, 0.0, 0.0]),
		(1_005.00, [11.3, 7.9, 5.7, 1.4, 0.0, 0.0]),
		(1_065.00, [12.1, 8.8, 6.5, 3.3, 0.0, 0.0]),
		(1_143.00, [13.1, 10.7, 8.3, 5.1, 2.7, 0.2]),
		(1_225.00, [14.1, 11.8, 9.3, 6.1, 3.6, 1.2]),
		(1_321.00, [15.2, 12.8, 10.5, 7.0, 4.6, 2.2]),
		(1_424.00, [16.2, 13.8, 11.4, 8.0, 6.5, 4.0]),
		(1_562.00, [17.2, 14.8, 12.3, 10.0, 7.5, 5.0]),
		(1_711.00, [18.6, 16.3, 14.8, 11.4, 8.9, 6.5]),
		(1_870.00, [19.9, 18.2, 17.3, 14.5, 12.5, 11.7]),
		(1_977.00, [20.9, 19.3, 18.2, 15.5, 14.5, 12.5]),
		(2_090.00, [21.9, 20.2, 19.2, 16.4, 15.5, 13.5]),
		(2_218.00, [22.8, 21.3, 20.3, 17.5, 16.5, 14.5]),
		(2_367.00, [23.8, 22.2, 21.3, 18.5, 17.6, 15.5]),
		(2_535.00, [24.8, 24.2, 22.2, 20.4, 18.5, 17.6]),
		(2_767.00, [25.8, 25.1, 23.3, 21.4, 19.4, 18.5]),
		(3_104.00, [27.0, 26.4, 24.5, 22.5, 20.6, 19.6]),
		(3_534.00, [28.6, 28.3, 26.8, 25.2, 24.6, 23.0]),
		(4_118.00, [29.7, 29.5, 27.7, 26.2, 25.6, 25.0]),
		(4_650.00, [31.4, 31.0, 29.4, 27.6, 27.0, 26.5]),
		(5_194.00, [32.3, 31.8, 31.3, 28.9, 28.0, 27.4]),
		(5_880.00, [33.3, 32.8, 32.2, 29.8, 29.2, 28.4]),
		(6_727.00, [35.3, 34.9, 34.1, 32.2, 31.8, 31.5]),
		(7_939.00, [36.3, 35.9, 35.5, 34.2, 32.8, 32.4]),
		(9_560.00, [38.2, 37.8, 37.4, 36.2, 35.8, 34.4]),
		(11_282.00, [39.2, 38.8, 38.4, 37.5, 36.7, 35.4]),
		(18_854.00, [40.2, 39.8, 39.4, 38.5, 38.1, 36.4]),
		(20_221.00, [41.2, 40.8, 40.4, 39.5, 39.1, 37.3]),
		(22_749.00, [41.9, 41.7, 41.4, 40.5, 40.1, 38.5]),
		(25_276.00, [42.9, 42.7, 42.3, 41.4, 41.1, 39.7]),
		(std::f64::INFINITY, [43.8, 43.6, 43.3, 42.4, 42.0, 40.7]),
	];

	pub const casado_titular_unico: [(Money, [Tax; 6]); 36] = [
		(710.00, [0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
		(740.00, [3.3, 0.0, 0.0, 0.0, 0.0, 0.0]),
		(754.00, [3.3, 0.0, 0.0, 0.0, 0.0, 0.0]),
		(794.00, [4.7, 0.9, 0.0, 0.0, 0.0, 0.0]),
		(836.00, [5.6, 1.8, 0.9, 0.0, 0.0, 0.0]),
		(886.00, [6.5, 3.7, 1.2, 0.0, 0.0, 0.0]),
		(974.00, [7.3, 4.6, 2.9, 0.0, 0.0, 0.0]),
		(1_081.00, [8.1, 5.5, 3.7, 1.0, 0.0, 0.0]),
		(1_225.00, [9.2, 6.9, 4.7, 2.0, 0.0, 0.0]),
		(1_404.00, [10.7, 8.9, 7.1, 4.3, 2.6, 1.7]),
		(1_629.00, [11.7, 10.0, 8.1, 6.3, 4.5, 2.7]),
		(1_733.00, [13.1, 11.4, 10.6, 7.7, 5.9, 5.1]),
		(1_849.00, [14.0, 12.4, 11.7, 9.0, 7.3, 6.5]),
		(1_998.00, [15.0, 13.3, 12.5, 10.0, 9.2, 7.4]),
		(2_157.00, [16.0, 14.3, 13.5, 10.9, 10.2, 8.5]),
		(2_347.00, [17.0, 16.3, 14.6, 11.9, 11.1, 9.5]),
		(2_566.00, [17.8, 17.2, 15.6, 13.8, 12.1, 11.4]),
		(2_934.00, [18.9, 18.2, 16.6, 14.8, 13.1, 12.3]),
		(3_356.00, [21.5, 21.4, 19.8, 18.4, 17.1, 16.7]),
		(3_611.00, [22.4, 22.3, 21.0, 19.4, 19.0, 17.6]),
		(3_882.00, [23.4, 23.3, 22.0, 20.6, 20.0, 18.6]),
		(4_210.00, [24.4, 24.3, 22.9, 21.6, 21.2, 20.6]),
		(4_604.00, [25.9, 25.3, 23.9, 22.5, 22.1, 21.8]),
		(5_076.00, [26.9, 26.3, 25.9, 23.5, 23.1, 22.7]),
		(5_654.00, [27.8, 27.2, 26.9, 24.5, 24.1, 23.7]),
		(6_381.00, [28.8, 28.2, 27.8, 25.5, 25.1, 24.7]),
		(7_323.00, [29.7, 29.6, 29.2, 27.0, 26.9, 26.7]),
		(8_441.00, [30.7, 30.6, 30.4, 29.0, 27.8, 27.6]),
		(9_336.00, [32.1, 32.0, 31.9, 30.7, 29.3, 29.1]),
		(10_448.00, [33.1, 33.0, 32.8, 31.7, 31.5, 30.0]),
		(14_013.00, [34.4, 34.3, 33.8, 32.6, 32.4, 31.3]),
		(20_118.00, [36.4, 36.3, 36.2, 35.1, 34.9, 33.7]),
		(22_749.00, [37.3, 37.2, 37.1, 36.5, 35.9, 34.7]),
		(25_276.00, [38.3, 38.2, 38.1, 37.4, 37.2, 35.7]),
		(28_309.00, [39.3, 39.2, 39.1, 38.4, 38.2, 37.0]),
		(std::f64::INFINITY, [40.3, 40.2, 40.1, 39.4, 39.2, 38.0]),
	];

	pub const casado_dois_titulares: [(Money, [Tax; 6]); 36] = [
		(710.00, [0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
		(720.00, [1.8, 1.7, 0.0, 0.0, 0.0, 0.0]),
		(740.00, [4.5, 3.4, 0.5, 0.0, 0.0, 0.0]),
		(754.00, [6.3, 3.7, 1.0, 0.0, 0.0, 0.0]),
		(814.00, [7.9, 5.1, 3.4, 2.5, 0.6, 0.0]),
		(922.00, [10.1, 7.3, 6.5, 3.8, 3.1, 1.2]),
		(1_005.00, [11.3, 8.6, 7.8, 5.1, 4.4, 3.1]),
		(1_065.00, [12.1, 9.5, 8.6, 6.0, 4.8, 3.9]),
		(1_143.00, [13.1, 11.4, 10.6, 7.9, 7.1, 5.3]),
		(1_225.00, [14.1, 12.4, 11.5, 8.9, 8.0, 6.3]),
		(1_321.00, [15.1, 14.4, 12.6, 10.7, 9.0, 8.1]),
		(1_424.00, [16.1, 15.3, 13.6, 11.9, 10.0, 9.2]),
		(1_562.00, [17.1, 16.4, 14.6, 12.8, 11.1, 10.2]),
		(1_711.00, [18.5, 17.7, 16.1, 14.3, 13.4, 11.7]),
		(1_870.00, [19.9, 19.3, 17.6, 16.0, 15.2, 13.5]),
		(1_977.00, [20.9, 20.4, 18.5, 16.9, 16.1, 14.5]),
		(2_090.00, [21.9, 21.4, 19.6, 17.7, 17.0, 16.3]),
		(2_218.00, [22.8, 22.3, 20.7, 18.9, 17.9, 17.3]),
		(2_367.00, [23.8, 23.4, 22.6, 19.9, 19.1, 18.2]),
		(2_535.00, [24.8, 24.4, 23.6, 21.0, 20.2, 19.4]),
		(2_767.00, [25.7, 25.2, 24.6, 21.9, 21.2, 20.4]),
		(3_104.00, [26.9, 26.5, 25.7, 23.1, 22.3, 21.6]),
		(3_534.00, [28.5, 28.4, 28.0, 25.7, 25.3, 24.9]),
		(4_118.00, [29.6, 29.5, 29.0, 27.6, 26.3, 25.9]),
		(4_650.00, [31.4, 31.1, 30.7, 29.0, 27.7, 27.3]),
		(5_194.00, [32.3, 32.0, 31.6, 30.3, 29.6, 28.3]),
		(5_880.00, [33.3, 33.0, 32.6, 31.3, 30.9, 29.2]),
		(6_727.00, [35.2, 35.0, 34.5, 33.8, 33.6, 33.4]),
		(7_939.00, [36.2, 36.0, 35.8, 34.7, 34.6, 34.4]),
		(9_560.00, [38.1, 37.9, 37.7, 36.6, 36.4, 36.3]),
		(11_282.00, [39.1, 38.9, 38.7, 38.0, 37.4, 37.2]),
		(18_854.00, [40.1, 39.9, 39.7, 39.0, 38.8, 38.2]),
		(20_221.00, [41.1, 40.9, 40.7, 40.0, 39.8, 39.2]),
		(22_749.00, [41.8, 41.7, 41.6, 41.0, 40.8, 40.4]),
		(25_276.00, [42.8, 42.7, 42.6, 41.9, 41.7, 41.5]),
		(std::f64::INFINITY, [43.8, 43.7, 43.6, 42.9, 42.7, 42.5]),
	];
}

mod ano_2021 {
	use super::{Money, Tax};
	pub const dependente_nao_casado: [(Money, [Tax; 6]); 35] = [
		(686.00, [0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
		(718.00, [4.0, 0.7, 0.0, 0.0, 0.0, 0.0]),
		(739.00, [7.2, 2.7, 0.0, 0.0, 0.0, 0.0]),
		(814.00, [8.0, 4.5, 1.0, 0.0, 0.0, 0.0]),
		(922.00, [10.2, 6.8, 3.5, 0.0, 0.0, 0.0]),
		(1_005.00, [11.4, 8.0, 5.7, 1.4, 0.0, 0.0]),
		(1_065.00, [12.2, 8.9, 6.6, 3.3, 0.0, 0.0]),
		(1_143.00, [13.2, 10.8, 8.4, 5.1, 2.7, 0.2]),
		(1_225.00, [14.2, 11.9, 9.4, 6.1, 3.6, 1.2]),
		(1_321.00, [15.3, 12.9, 10.6, 7.1, 4.6, 2.2]),
		(1_424.00, [16.3, 13.9, 11.5, 8.1, 6.6, 4.0]),
		(1_562.00, [17.3, 14.9, 12.4, 10.1, 7.6, 5.0]),
		(1_711.00, [18.7, 16.4, 14.9, 11.5, 9.0, 6.6]),
		(1_870.00, [20.1, 18.3, 17.4, 14.6, 12.6, 11.8]),
		(1_977.00, [21.1, 19.5, 18.3, 15.6, 14.6, 12.6]),
		(2_090.00, [22.1, 20.4, 19.4, 16.5, 15.6, 13.6]),
		(2_218.00, [23.0, 21.5, 20.5, 17.6, 16.6, 14.6]),
		(2_367.00, [24.0, 22.4, 21.5, 18.6, 17.7, 15.6]),
		(2_535.00, [25.0, 24.4, 22.4, 20.6, 18.6, 17.7]),
		(2_767.00, [26.0, 25.3, 23.5, 21.6, 19.6, 18.6]),
		(3_104.00, [27.2, 26.6, 24.7, 22.7, 20.8, 19.8]),
		(3_534.00, [28.8, 28.5, 27.0, 25.4, 24.8, 23.2]),
		(4_118.00, [29.9, 29.7, 27.9, 26.4, 25.8, 25.2]),
		(4_650.00, [31.7, 31.2, 29.6, 27.8, 27.2, 26.7]),
		(5_194.00, [32.6, 32.1, 31.6, 29.1, 28.2, 27.6]),
		(5_880.00, [33.6, 33.1, 32.5, 30.0, 29.4, 28.6]),
		(6_727.00, [35.6, 35.2, 34.4, 32.5, 32.1, 31.8]),
		(7_939.00, [36.6, 36.2, 35.8, 34.5, 33.1, 32.7]),
		(9_560.00, [38.5, 38.1, 37.7, 36.5, 36.1, 34.7]),
		(11_282.00, [39.5, 39.1, 38.7, 37.8, 37.0, 35.7]),
		(18_854.00, [40.5, 40.1, 39.7, 38.8, 38.4, 36.7]),
		(20_221.00, [41.5, 41.1, 40.7, 39.8, 39.4, 37.6]),
		(22_749.00, [42.2, 42.0, 41.7, 40.8, 40.4, 38.8]),
		(25_276.00, [43.2, 43.0, 42.6, 41.7, 41.4, 40.0]),
		(std::f64::INFINITY, [44.2, 44.0, 43.6, 42.7, 42.3, 41.0]),
	];
}

mod ano_2020 {
	use super::{Money, Tax};
	pub const dependente_nao_casado: [(Money, [Tax; 6]); 36] = [
		(659.00, [0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
		(686.00, [0.1, 0.0, 0.0, 0.0, 0.0, 0.0]),
		(718.00, [4.2, 0.8, 0.0, 0.0, 0.0, 0.0]),
		(739.00, [7.3, 2.8, 0.2, 0.0, 0.0, 0.0]),
		(814.00, [8.2, 4.6, 1.1, 0.0, 0.0, 0.0]),
		(922.00, [10.4, 6.9, 3.6, 0.0, 0.0, 0.0]),
		(1_005.00, [11.6, 8.2, 5.8, 1.5, 0.0, 0.0]),
		(1_065.00, [12.4, 9.1, 6.7, 3.4, 0.0, 0.0]),
		(1_143.00, [13.5, 11.0, 8.6, 5.2, 2.8, 0.3]),
		(1_225.00, [14.5, 12.1, 9.6, 6.2, 3.7, 1.3]),
		(1_321.00, [15.6, 13.2, 10.8, 7.2, 4.7, 2.3]),
		(1_424.00, [16.6, 14.2, 11.7, 8.3, 6.7, 4.1]),
		(1_562.00, [17.7, 15.2, 12.7, 10.3, 7.8, 5.1]),
		(1_711.00, [19.1, 16.7, 15.2, 11.7, 9.2, 6.7]),
		(1_870.00, [20.5, 18.7, 17.8, 14.9, 12.9, 12.0]),
		(1_977.00, [21.5, 19.9, 18.7, 15.9, 14.9, 12.9]),
		(2_090.00, [22.5, 20.8, 19.8, 16.8, 15.9, 13.9]),
		(2_218.00, [23.5, 21.9, 20.9, 18.0, 16.9, 14.9]),
		(2_367.00, [24.5, 22.9, 21.9, 19.0, 18.1, 15.9]),
		(2_535.00, [25.5, 24.9, 22.9, 21.0, 19.0, 18.1]),
		(2_767.00, [26.5, 25.8, 24.0, 22.0, 20.0, 19.0]),
		(3_104.00, [27.8, 27.1, 25.2, 23.2, 21.2, 20.2]),
		(3_534.00, [29.4, 29.1, 27.5, 25.9, 25.3, 23.7]),
		(4_118.00, [30.5, 30.3, 28.5, 26.9, 26.3, 25.7]),
		(4_650.00, [32.3, 31.8, 30.2, 28.4, 27.8, 27.2]),
		(5_194.00, [33.3, 32.8, 32.2, 29.7, 28.8, 28.2]),
		(5_880.00, [34.3, 33.8, 33.2, 30.6, 30.0, 29.2]),
		(6_727.00, [36.3, 35.9, 35.1, 33.2, 32.8, 32.4]),
		(7_939.00, [37.3, 36.9, 36.5, 35.2, 33.8, 33.4]),
		(9_560.00, [39.3, 38.9, 38.5, 37.2, 36.8, 35.4]),
		(11_282.00, [40.3, 39.9, 39.5, 38.6, 37.8, 36.4]),
		(18_854.00, [41.3, 40.9, 40.5, 39.6, 39.2, 37.4]),
		(20_221.00, [42.3, 41.9, 41.5, 40.6, 40.2, 38.4]),
		(22_749.00, [43.1, 42.9, 42.5, 41.6, 41.2, 39.6]),
		(25_276.00, [44.1, 43.9, 43.5, 42.6, 42.2, 40.8]),
		(std::f64::INFINITY, [45.1, 44.9, 44.5, 43.6, 43.2, 41.8]),
	];
}

mod ano_2019 {
	use super::{Money, Tax};
	pub const dependente_nao_casado: [(Money, [Tax; 6]); 36] = [
		(654.00, [0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
		(683.00, [0.2, 0.0, 0.0, 0.0, 0.0, 0.0]),
		(715.00, [4.4, 1.0, 0.0, 0.0, 0.0, 0.0]),
		(736.00, [7.4, 2.9, 0.3, 0.0, 0.0, 0.0]),
		(811.00, [8.3, 4.7, 1.2, 0.0, 0.0, 0.0]),
		(919.00, [10.5, 7.0, 3.7, 0.1, 0.0, 0.0]),
		(1001.00, [11.7, 8.3, 5.9, 1.6, 0.0, 0.0]),
		(1061.00, [12.5, 9.2, 6.8, 3.5, 0.0, 0.0]),
		(1139.00, [13.6, 11.1, 8.7, 5.3, 2.9, 0.4]),
		(1221.00, [14.6, 12.2, 9.7, 6.3, 3.8, 1.4]),
		(1317.00, [15.7, 13.3, 10.9, 7.3, 4.8, 2.4]),
		(1419.00, [16.7, 14.3, 11.8, 8.4, 6.8, 4.2]),
		(1557.00, [17.8, 15.3, 12.8, 10.4, 7.9, 5.2]),
		(1705.00, [19.2, 16.8, 15.3, 11.8, 9.3, 6.8]),
		(1864.00, [20.6, 18.8, 17.9, 15.0, 13.0, 12.1]),
		(1971.00, [21.6, 20.0, 18.8, 16.0, 15.0, 13.0]),
		(2083.00, [22.6, 20.9, 19.9, 16.9, 16.0, 14.0]),
		(2211.00, [23.6, 22.0, 21.0, 18.1, 17.0, 15.0]),
		(2359.00, [24.6, 23.0, 22.0, 19.1, 18.2, 16.0]),
		(2527.00, [25.6, 25.0, 23.0, 21.1, 19.1, 18.2]),
		(2758.00, [26.6, 25.9, 24.1, 22.1, 20.1, 19.1]),
		(3094.00, [27.9, 27.2, 25.3, 23.3, 21.3, 20.3]),
		(3523.00, [29.5, 29.2, 27.6, 26.0, 25.4, 23.8]),
		(4105.00, [30.7, 30.5, 28.6, 27.0, 26.4, 25.8]),
		(4636.00, [32.5, 32.0, 30.4, 28.5, 27.9, 27.3]),
		(5178.00, [33.5, 33.0, 32.4, 29.8, 28.9, 28.3]),
		(5862.00, [34.5, 34.0, 33.4, 30.8, 30.2, 29.3]),
		(6706.00, [36.5, 36.1, 35.3, 33.4, 33.0, 32.6]),
		(7915.00, [37.5, 37.1, 36.7, 35.4, 34.0, 33.6]),
		(9531.00, [39.5, 39.1, 38.7, 37.4, 37.0, 35.6]),
		(11248.00, [40.5, 40.1, 39.7, 38.8, 38.0, 36.6]),
		(18797.00, [41.5, 41.1, 40.7, 39.8, 39.4, 37.6]),
		(20160.00, [42.5, 42.1, 41.7, 40.8, 40.4, 38.6]),
		(22680.00, [43.3, 43.1, 42.7, 41.8, 41.4, 39.8]),
		(25200.00, [44.3, 44.1, 43.7, 42.8, 42.4, 41.0]),
		(std::f64::INFINITY, [45.3, 45.1, 44.7, 43.8, 43.4, 42.0]),
	];
}