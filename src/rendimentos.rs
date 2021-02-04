use crate::{ss, Money};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Deslocacao {
    InternacionalDirector,
    InternacionalOutros,
    NacionalDirector,
    NacionalOutros,
}

pub trait Rendimento: RendimentoBoxClone {
    fn valor(&self) -> Money;
}

// Splitting RendimentoClone into its own trait allows us to provide a blanket
// implementation for all compatible types, without having to implement the
// rest of Rendimento.  In this case, we implement it for all types that have
// 'static lifetime (*i.e.* they don't contain non-'static pointers), and
// implement both Rendimento and Clone. Don't ask me how the compiler resolves
// implementing RendimentoClone for Rendimento when Rendimento requires
// RendimentoClone; I have *no* idea why this works.
pub trait RendimentoBoxClone {
    fn clone_box(&self) -> Box<dyn Rendimento>;
}

impl<T> RendimentoBoxClone for T
where
    T: 'static + Rendimento + Clone,
{
    fn clone_box(&self) -> Box<dyn Rendimento> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Rendimento> {
    fn clone(&self) -> Box<dyn Rendimento> {
        self.clone_box()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Salario(Money, f64);
impl Rendimento for Salario {
    fn valor(&self) -> Money {
        self.0 * self.1
    }
}

/*fn teste() {
    let rends: Vec<Box<dyn Rendimento>> = vec![];
}*/

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rendimento_ {
    /// Base mensal do salário (Categoria A) e duração em meses
    Salario(Money, f64),
    /// Base diária do subsídio, duração em dias e se é pago em vales
    Refeicao(Money, u32, bool),
    /// Subsídio de natal e férias de um salário
    NatalFerias(Money),
    /// Ajuda de custo para deslocações
    AjudaCustoDeslocacao(Money, Deslocacao),
    /// Pagamento de km's feitos em carro próprio (preço ao km, km's)
    KmsCarroProprio(Money, f64),
    // TODO: incluir ValeInfancia (muito vantajoso para empresa e trabalhadores)

    /* TODO: incluir Indemnização cessação de trabalho
    (isento até Remuneração média dos últimos 12 meses * anos trabalho)
    Tratando-se de gestor, administrador, gerente de pessoa coletiva, gerente público ou representante de
    estabelecimento estável de entidade não residente, os montantes recebidos pela cessação do vínculo laboral
    são sujeitos a tributação na sua totalidade, apenas na parte respeitante a essas mesmas funções.
    A parte respeitante a períodos em que estes tenham exercido funções como trabalhador por conta de outrem
    continuam a beneficiar da exclusão de tributação. Relativamente à Segurança Social, não constituem base de
    incidência a compensação por cessação do contrato de trabalho no caso de despedimento coletivo; por
    extinção do posto de trabalho, por inadaptação; por não concessão de aviso prévio; por caducidade; por
    resolução por parte do trabalhador; por cessação antes de findo o prazo convencional do contrato de
    trabalho a prazo.
    */
}

impl Rendimento_ {
    const salario_factor_empresa: f64 = 1.0 + ss::tax::empresa + 0.01;

    pub fn salario_custo_empresa(custo_mensal: Money, meses: f64) -> [Rendimento_; 2] {
        Rendimento_::salario_bruto(custo_mensal / Rendimento_::salario_factor_empresa, meses)
    }

    pub fn salario_bruto(bruto_mensal: Money, meses: f64) -> [Rendimento_; 2] {
        let salario = Rendimento_::Salario(bruto_mensal, meses);
        let subsidios = salario.valor() / 6.0;
        [salario, Rendimento_::NatalFerias(subsidios)]
    }

    pub fn valor(&self) -> Money {
        match *self {
            Rendimento_::Salario(valor, meses) => valor * meses,
            Rendimento_::NatalFerias(valor) => valor,
            Rendimento_::Refeicao(valor, dias, _) => valor * f64::from(dias),
            Rendimento_::AjudaCustoDeslocacao(v, _) => v,
            Rendimento_::KmsCarroProprio(pkm, km) => pkm * km,
        }
    }

    pub fn custo_empresa(&self) -> Money {
        match *self {
            Rendimento_::Salario(_, _) => self.valor() * Rendimento_::salario_factor_empresa,
            Rendimento_::NatalFerias(valor) => valor * Rendimento_::salario_factor_empresa,
            Rendimento_::Refeicao(_, _, _) => self.valor(),
            Rendimento_::AjudaCustoDeslocacao(v, _) => v,
            Rendimento_::KmsCarroProprio(pkm, km) => pkm * km,
        }
    }
}
