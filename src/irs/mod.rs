use crate::rendimentos::{Deslocacao, Rendimento, Rendimento_};
use crate::{diferenca_positiva, Money};

pub mod retencao;

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

pub mod categoria {
    pub trait Categoria {}

    pub trait Rendimento<C>: RendimentoBoxClone<C> + crate::rendimentos::Rendimento
    where
        C: Categoria,
    {
        type Categoria;
    }

    pub trait RendimentoBoxClone<C> {
        fn clone_box(&self) -> Box<dyn Rendimento<C, Categoria = C>>;
    }

    impl<C, T> RendimentoBoxClone<C> for T
    where
        T: 'static + Rendimento<C, Categoria = C> + Clone,
        C: Categoria,
    {
        fn clone_box(&self) -> Box<dyn Rendimento<C, Categoria = C>> {
            Box::new(self.clone())
        }
    }

    impl Rendimento<A> for crate::rendimentos::Salario {
        type Categoria = A;
    }

    macro_rules! categorias {
        ($(#[$docs:meta] $cat:ident),+) => {
            $(
                #[$docs] pub struct $cat;
                impl Categoria for $cat {}
            )+

            pub use rendimentos::ApensadorDeRendimento;
            pub use rendimentos::Categorizados as RendimentosCategorizados;

            mod rendimentos {
                pub trait ApensadorDeRendimento<T, C>
                where
                    T: super::Rendimento<C, Categoria = C>,
                    C: super::Categoria
                {
                    fn push(&mut self, rend: T);
                }

                paste::item! {
                    $(
                        pub trait [<RendimentoCat $cat>] = super::Rendimento<super::$cat, Categoria = super::$cat>;
                        impl<T> ApensadorDeRendimento<T, super::$cat> for Categorizados
                        where
                            T: [<RendimentoCat $cat>] + 'static,
                        {
                            fn push(&mut self, rend: T) {
                                self.[<cat_ $cat>].push(Box::new(rend));
                            }
                        }

                        impl Clone for Box<dyn [<RendimentoCat $cat>]> {
                            fn clone(&self) -> Self {
                                super::RendimentoBoxClone::clone_box(&**self)
                            }
                        }
                    )+

                    #[allow(non_snake_case)]
                    #[derive(Clone)]
                    pub struct Categorizados {
                        $([<cat_ $cat>]: Vec<Box<dyn [<RendimentoCat $cat>]>>,)+
                    }

                    impl Default for Categorizados {
                        fn default() -> Self {
                            Categorizados {
                                $([<cat_ $cat>]: Vec::default(),)+
                            }
                        }
                    }
                }
            }
        }
    }

    categorias! {
    /// Rendimento de trabalho dependente
    A,
    /// Rendimento empresarial e profissional
    B,
    /// Rendimento de capital
    E,
    /// Rendimento predial
    F,
    /// Incremento patrimonial
    G,
    /// Pensão
    H
    }
}

pub mod limites {
    use crate::{MoneyDaily22, MoneyYearly};
    pub const subsidio_refeição: MoneyDaily22 = 4.77;
    pub const vale_refeição: MoneyDaily22 = 7.63;
    pub const ajudas_custo_dia: MoneyDaily22 = 50.20;
    pub const isencao_retencao_cat_b: MoneyYearly = 10000.0;
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

// 2019
const escalao_2019: [(f64, f64); 7] = [
    (07_091.00, 0.145),
    (10_700.00, 0.230),
    (20_261.00, 0.285),
    (25_000.00, 0.350),
    (36_856.00, 0.370),
    (80_640.00, 0.450),
    (std::f64::INFINITY, 0.48),
];

pub fn escalao(rendimento_colectavel: f64) -> (usize, (f64, f64, f64)) {
    escalao_2019
        .iter()
        .scan(
            (0.0, 0.0, 0.001),
            |(lim, tax, a_abater), (new_lim, new_tax)| {
                if rendimento_colectavel < *lim {
                    //println!("{}º escalao (<{:.2}): {:.2} * {:.2}% - {:.2}", i + 1, lim, rend_coletavel, tax * 100.0, a_abater);
                    return None;
                }
                *a_abater += *lim * new_tax - *lim * *tax;
                *lim = *new_lim;
                *tax = *new_tax;
                Some((*new_lim, *new_tax, *a_abater))
            },
        )
        .enumerate()
        .last()
        .unwrap()
}

pub struct Irs {}

use crate::ElementoFamiliar;
use categoria::RendimentosCategorizados;

#[derive(Default)]
pub struct IrsBuilder {
    elementos: Vec<ElementoFamiliar>,
    rendimentos: Vec<RendimentosCategorizados>,
}

impl IrsBuilder {
    pub fn novo_elemento(
        mut self,
        elemento: ElementoFamiliar,
        rendimentos: RendimentosCategorizados,
    ) -> IrsBuilder {
        self.elementos.push(elemento);
        self.rendimentos.push(rendimentos);
        self
    }

    pub fn fazer() -> Irs {
        Irs {}
    }
}

pub fn novo_irs() -> IrsBuilder {
    IrsBuilder::default()
}

/*pub mod categoria {
    pub trait Categoria {}

    pub trait Rendimento: crate::rendimentos::Rendimento {
        type Categoria: Categoria;
    }

    impl Rendimento for crate::rendimentos::Salario {
        type Categoria = A;
    }

    macro_rules! categorias {
        ($(#[$docs:meta] $cat:ident),+) => {
            $(
                #[$docs] pub struct $cat;
                impl Categoria for $cat {}
            )+
            pub mod rendimentos {
                pub trait Pushable<T, C>
                where
                    T: super::Rendimento<Categoria = C>,
                    C: super::Categoria,
                {
                    fn push(&mut self, rend: T);
                }

                paste::item! {
                    $(
                        pub trait [<RendimentoCat $cat>] = super::Rendimento<Categoria = super::$cat>;
                        impl<T> Pushable<T, super::$cat> for Categorizados
                        where
                            T: 'static + [<RendimentoCat $cat>],
                        {
                            fn push(&mut self, rend: T) {
                                self.[<cat_ $cat>].push(Box::new(rend));
                            }
                        }

                        /*pub trait [<RendimentoCat $cat Clone>] {
                            fn clone_box(&self) -> Box<dyn [<RendimentoCat $cat>]>;
                        }

                        impl<T> [<RendimentoCat $cat Clone>] for T
                        where
                            T: 'static + [<RendimentoCat $cat>] + Clone,
                        {
                            fn clone_box(&self) -> Box<dyn [<RendimentoCat $cat>]> {
                                Box::new(self.clone())
                            }
                        }

                        impl Clone for [<RendimentoCat $cat>] {
                            fn clone(&self) -> Self {
                                [<RendimentoCat $cat Clone>]::clone_box(&**self)
                            }
                        }

                        impl Clone for Box<dyn [<RendimentoCat $cat>]> {
                            fn clone(&self) -> Self {
                                [<RendimentoCat $cat Clone>]::clone_box(&**self)
                            }
                        }*/

                        /*impl<T> Clone for Box<T>
                        where
                            T: 'static + [<RendimentoCat $cat>] + Clone,
                        {
                            fn clone(&self) -> Self {
                                [<RendimentoCat $cat Clone>]::clone_box(&**self)
                            }
                        }*/
                    )+

                    /*impl<C> Clone for Box<dyn super::Rendimento<Category = C>>
                    where
                        C: Categoria,
                    {
                        fn clone(&self) -> Self {
                            [<RendimentoCat $cat Clone>]::clone_box(&**self)
                        }
                    }*/


                    #[allow(non_snake_case)]
                    //#[derive(Clone)]
                    pub struct Categorizados {
                        $([<cat_ $cat>]: Vec<Box<dyn [<RendimentoCat $cat>]>>,)+
                    }

                    impl Default for Categorizados {
                        fn default() -> Self {
                            Categorizados {
                                $([<cat_ $cat>]: Vec::default(),)+
                            }
                        }
                    }
                }
            }
        }
    }

    categorias! {
    /// Rendimento de trabalho dependente
    A,
    /// Rendimento empresarial e profissional
    B,
    /// Rendimento de capital
    E,
    /// Rendimento predial
    F,
    /// Incremento patrimonial
    G,
    /// Pensão
    H
    }
}*/
