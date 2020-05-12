use crate::definitions::Money;
use crate::time::Date;
use std::collections::HashMap;

pub trait PricingEngine {
    type R: Results;
    type A: Arguments;

    fn get_results(&self) -> Self::R;
    fn get_arguments(&self) -> Self::A;
    fn reset(&self);
    fn update(&self);
    fn calculate(&self);
}

pub trait Results {
    fn reset(&mut self);
    fn get(&self) -> &BaseResults;
}

/// Results.
/// ============================
///
///
/// BaseResults is a base class for pricing engine results.
pub struct BaseResults {
    pub value: Money,
    pub error_estimate: Money,
    pub valuation_date: Date,
    pub additional_results: HashMap<String, Money>,
}
impl Results for BaseResults {
    fn reset(&mut self) {
        self.valuation_date = Date::default();
        self.value = Money::default();
        self.error_estimate = Money::default();
        self.additional_results.clear();
    }
    fn get(&self) -> &BaseResults {
        self
    }
}

pub trait Arguments {
    fn validate(&self);
}
