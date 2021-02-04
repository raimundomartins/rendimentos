use crate::rendimentos::Rendimento;
use crate::{diferenca_positiva, Money};

pub mod tax {
    use crate::Tax;
    pub const trabalhador: Tax = 0.11;
    pub const empresa: Tax = 0.2375;
    pub const cat_b: Tax = 0.7 * 0.214;
    pub const cat_a: Tax = trabalhador + empresa;
}

#[allow(non_snake_case)]
pub mod limites {
    use crate::{MoneyMonthly, MoneyYearly};
    pub const fn isencao_cat_a_cat_b(IAS: MoneyMonthly) -> MoneyMonthly {
        4.0 * IAS
    }
    pub const fn base_incidencia(IAS: MoneyMonthly) -> MoneyYearly {
        12.0 * IAS
    } // Whatis?
    pub const fn serv_min_entidade_contratante(IAS: MoneyMonthly) -> MoneyMonthly {
        6.0 * IAS
    }
}

pub trait Tributavel {
    fn colectavel(&self) -> Money;
}
