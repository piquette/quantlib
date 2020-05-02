use super::money::Money;
use crate::pricingengines::PricingEngine;
use std::collections::HashMap;

pub trait Instrument {
    fn is_expired() -> bool;
    fn set_pricing_engine<E: PricingEngine>(engine: E);
    fn npv() -> Money;
    fn error_estimate() -> Money;
    fn setup_expired();
    fn setup_arguments();
    fn perform_calculations();
    fn fetch_results();
    fn valuation_date();
    fn result();
    fn additional_results() -> HashMap<String, Money>;
}
