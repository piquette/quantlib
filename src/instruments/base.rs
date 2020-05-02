use super::money::Money;
use crate::pricingengines::PricingEngine;
use crate::time::Date;
use std::collections::HashMap;

pub struct Base {
    npv: Money,
    error_estimate: Money,
    valuation_date: Date,
    additional_results: HashMap<String, Money>,
    engine: Box<dyn PricingEngine>,
}

impl Base {
    pub fn calculate(&self) {}
    pub fn setup_expired(&self) {}
    pub fn perform_calculations(&self) {}
}
