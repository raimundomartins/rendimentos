pub mod tax {
	use crate::Tax;
	pub const trabalhador: Tax = 0.11;
	pub const empresa: Tax = 0.2375;
	pub const cat_b: Tax = 0.7 * 0.214;
	pub const cat_a: Tax = trabalhador + empresa;
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
