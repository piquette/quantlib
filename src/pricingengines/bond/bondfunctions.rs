use crate::cashflows as cf;
pub use crate::cashflows::{CashFlow, Leg};
use crate::instruments::bond::Bond;
use crate::pricingengines::PricingEngine;
pub use crate::time::traits::Calendar as Cal;
use crate::time::Date;

pub fn start_date<C: Cal, CF: CashFlow, PE: PricingEngine>(bond: Bond<C, CF, PE>) -> Date {
    cf::start_date(bond.cashflows)
}

pub fn maturity_date<C: Cal, CF: CashFlow, PE: PricingEngine>(bond: Bond<C, CF, PE>) -> Date {
    cf::maturity_date(bond.cashflows)
}

pub fn is_tradeable<C: Cal, CF: CashFlow, PE: PricingEngine + Default>(
    bond: Bond<C, CF, PE>,
    mut settlement_date: Date,
) -> bool {
    if settlement_date == Date::default() {
        settlement_date = bond.settlement_date(None);
    }
    bond.notional(Some(settlement_date)) != 0.0
}
