use super::super::bond::Bond;
use crate::cashflows::CashFlow;
use crate::pricingengines::PricingEngine;
use crate::time::traits::Calendar as Cal;
use crate::time::{DayCounter, Frequency};

pub struct FixedRateBond<C: Cal, CF: CashFlow, DC: DayCounter, PE: PricingEngine> {
    pub bond: Bond<C, CF, PE>,
    pub frequency: Frequency,
    pub day_counter: DC,
}
