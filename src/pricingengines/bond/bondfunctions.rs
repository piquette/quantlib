use crate::cashflows as cf;
pub use crate::cashflows::{CashFlow, Leg};
use crate::definitions::Rate;
use crate::instruments::bond::Bond;
use crate::pricingengines::PricingEngine;
use crate::termstructures::Compounding;
pub use crate::time::traits::Calendar as Cal;
use crate::time::traits::DayCounter;
use crate::time::Date;
use crate::time::Frequency;

pub fn start_date<C: Cal, CF: CashFlow, PE: PricingEngine>(bond: &Bond<C, CF, PE>) -> Date {
    cf::start_date(&bond.cashflows)
}

pub fn maturity_date<C: Cal, CF: CashFlow, PE: PricingEngine>(bond: &Bond<C, CF, PE>) -> Date {
    cf::maturity_date(&bond.cashflows)
}

pub fn is_tradeable<C: Cal, CF: CashFlow, PE: PricingEngine + Default>(
    bond: &Bond<C, CF, PE>,
    mut settlement_date: Date,
) -> bool {
    if settlement_date == Date::default() {
        settlement_date = bond.settlement_date(None);
    }
    bond.notional(Some(settlement_date)) != 0.0
}

pub fn yield_with<C: Cal, CF: CashFlow, PE: PricingEngine, DC: DayCounter>(
    bond: &Bond<C, CF, PE>,
    clean_price: f64,
    day_counter: DC,
    comp: Compounding,
    freq: Frequency,
    settlement_date: Date,
    accuracy: f64,
    max_evaluations: usize,
) -> f64 {
    0.0
}

pub fn clean_price<C: Cal, CF: CashFlow, PE: PricingEngine, DC: DayCounter>(
    bond: Bond<C, CF, PE>,
    y: Rate,
    day_counter: DC,
    comp: Compounding,
    freq: Frequency,
    settlement: Date,
) -> f64 {
    0.0
}

pub fn accrued_amount<C: Cal, CF: CashFlow, PE: PricingEngine>(
    bond: &Bond<C, CF, PE>,
    settlement_date: Date,
) -> f64 {
    0.0
}

pub fn next_coupon_rate<C: Cal, CF: CashFlow, PE: PricingEngine>(
    bond: &Bond<C, CF, PE>,
    settlement_date: Date,
) -> Rate {
    0.0
}

pub fn previous_coupon_rate<C: Cal, CF: CashFlow, PE: PricingEngine>(
    bond: &Bond<C, CF, PE>,
    settlement_date: Date,
) -> Rate {
    0.0
}

pub fn next_cashflow_date<C: Cal, CF: CashFlow, PE: PricingEngine>(
    bond: &Bond<C, CF, PE>,
    settlement_date: Date,
) -> Date {
    Date::default()
}

pub fn previous_cashflow_date<C: Cal, CF: CashFlow, PE: PricingEngine>(
    bond: &Bond<C, CF, PE>,
    settlement_date: Date,
) -> Date {
    Date::default()
}
