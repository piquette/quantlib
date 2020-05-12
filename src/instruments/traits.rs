use crate::definitions::Money;
use crate::time::Date;
use crate::pricingengines::{Arguments, PricingEngine, Results};
use std::collections::HashMap;

/// Instrument trait.
/// This trait is purely abstract and defines the interface of concrete
/// instruments which will be derived from this one.
pub trait Instrument {
    type E: PricingEngine;
    /// returns the net present value of the instrument.
    fn npv(&mut self) -> Money;
    /// returns the error estimate on the NPV when available.
    fn error_estimate(&mut self) -> Money;
    /// returns the date the net present value refers to.
    fn valuation_date(&mut self) -> Date;
    /// returns any additional result returned by the pricing engine.
    fn result(&mut self, tag: String) -> Result<Money, &str>;
    /// returns any additional result returned by the pricing engine.
    fn additional_results(&self) -> &HashMap<String, Money>;
    /// returns whether the instrument might have value greater than zero.
    fn is_expired(&self) -> bool;
    /// set the pricing engine to be used.
    fn set_pricing_engine(&mut self, engine: Self::E);
    /// When a derived argument structure is defined for an
    /// instrument, this method should be overridden to fill
    /// it. This is mandatory in case a pricing engine is used.
    fn setup_arguments<A: Arguments>(&self, args: A);
    /// When a derived result structure is defined for an
    /// instrument, this method should be overridden to read from
    /// it. This is mandatory in case a pricing engine is used.
    fn fetch_results<R: Results>(&mut self, results: R);
    ///
    fn calculate(&mut self);
    ///
    fn setup_expired(&mut self);
    ///
    fn perform_calculations(&mut self);
}
