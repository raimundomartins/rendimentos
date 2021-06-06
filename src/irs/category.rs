pub trait Categoria {}

pub trait Rendimento<C>: RendimentoBoxClone<C>
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
	fn clone_box(&self) -> Box<dyn Rendimento<C, Categoria = C>> { Box::new(self.clone()) }
}

impl Rendimento<A> for crate::income::Income {
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
