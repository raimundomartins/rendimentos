mod money;
mod quantity_per_time;
mod yearly_plan;

pub use money::Money;
pub use quantity_per_time::{Hourly, Monthly, QuantityPerTime, Workdaily, Yearly};
pub use yearly_plan::YearlyPlan;

pub type TaxRate = f64;
pub type MoneyRate<P> = QuantityPerTime<Money, P>;
