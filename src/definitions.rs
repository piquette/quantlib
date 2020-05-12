use crate::currencies::Currency;

pub type Time = f64;
pub type DiscountFactor = f64;
pub type Rate = f64;

/// A return type that contains a value denoted in a currency.
#[derive(Default, Copy, Clone, PartialEq)]
pub struct Money {
    pub value: f64,
    pub currency: Option<Currency>,
}

//impl Default for M
