pub mod brackets;
pub mod withholding;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Taxing {
	None,
	Taxed,
	TaxedWitheld,
}

pub enum EstatutoFiscal {
	/// Tributado pela globalidade dos rendimentos obtidos (em Portugal e no
	/// estrangeiro)
	Residente,
	/// Exclusão de tributação de 50% dos rendimentos do trabalho dependente
	/// e dos rendimentos empresariais e profissionais.
	ExResidente,
	/// Tributado pelos rendimentos líquidos do trabalho dependente e
	/// independente, a uma taxa fixa de 20%, relativamente aos rendimentos
	/// derivados de atividades de "elevado valor acrescentado". Rendimentos
	/// de fonte estrangeira podem ficar isentos de tributação, em
	/// determinadas circunstâncias.
	ResidenteNãoHabitual,
	/// Tributado pelos rendimentos obtidos em Portugal.
	NãoResidente,
}
