pub mod tax {
	use crate::TaxRate;
	pub const TRABALHADOR: TaxRate = 0.11;
	pub const EMPRESA: TaxRate = 0.2375;
	pub const FUNDO_GARANTIA_SALARIAL: TaxRate = 0.01;
	pub const CAT_B: TaxRate = 0.7 * 0.214;
	pub const CAT_A: TaxRate = TRABALHADOR + EMPRESA;
}

use derive_builder::Builder;

use crate::TaxRate;

#[derive(Builder, Debug, Default, Clone, PartialEq)]
pub struct Taxes {
	#[builder(default = "0.01")]
	pub salary_guarantee_fund_tax: TaxRate,
	#[builder(default = "0.2375")]
	pub company_tsu: TaxRate,
	#[builder(default = "0.11")]
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

/*#[allow(non_snake_case)]
pub mod limites {
	pub const fn isencao_cat_a_cat_b(IAS: MoneyMonthly) -> MoneyMonthly { 4.0 * IAS }
	pub const fn base_incidencia(IAS: MoneyMonthly) -> MoneyYearly { 12.0 * IAS } // Whatis?
	pub const fn serv_min_entidade_contratante(IAS: MoneyMonthly) -> MoneyMonthly { 6.0 * IAS }
}*/

#[derive(Clone, Copy, Debug)]
pub enum SegSocVarCatB {
	M25 = -25,
	M20 = -20,
	M15 = -15,
	M10 = -10,
	M05 = -5,
	Zero = 0,
	P05 = 5,
	P10 = 10,
	P15 = 15,
	P20 = 20,
	P25 = 25,
}
