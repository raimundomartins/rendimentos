pub mod brackets;
pub mod category;
pub mod withholding;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Taxing {
	None,
	Taxed,
	TaxedWitheld,
}

/*pub trait Tributavel: Clone + PartialEq {
	fn colectavel(&self) -> Money;
}

impl Tributavel for Rendimento_ {
	fn colectavel(&self) -> Money {
		match *self {
			Rendimento::Salario(valor, meses) => valor * meses,
			Rendimento::NatalFerias(valor) => valor,
			Rendimento::Refeicao(valor, dias, _) => valor * f64::from(dias),
			Rendimento::AjudaCustoDeslocacao(v, d) => diferenca_positiva(
				v,
				match d {
					Deslocacao::InternacionalDirector => 100.24,
					Deslocacao::InternacionalOutros => 89.35,
					Deslocacao::NacionalDirector => 69.19,
					Deslocacao::NacionalOutros => 50.20,
				},
			),
			Rendimento::KmsCarroProprio(pkm, km) => diferenca_positiva(pkm, 0.36) * km,
		}
	}
}*/

pub struct Irs {}

use category::RendimentosCategorizados;

use crate::ElementoFamiliar;

#[derive(Default)]
pub struct IrsBuilder {
	elementos: Vec<ElementoFamiliar>,
	rendimentos: Vec<RendimentosCategorizados>,
}

impl IrsBuilder {
	pub fn novo_elemento(
		mut self, elemento: ElementoFamiliar, rendimentos: RendimentosCategorizados,
	) -> IrsBuilder {
		self.elementos.push(elemento);
		self.rendimentos.push(rendimentos);
		self
	}

	pub fn fazer() -> Irs { Irs {} }
}

pub fn novo_irs() -> IrsBuilder { IrsBuilder::default() }

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
