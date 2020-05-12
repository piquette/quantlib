use super::traits::Instrument;
use crate::definitions::Money;
use crate::patterns::LazyObject;
use crate::pricingengines::{Arguments, PricingEngine, Results};
use crate::time::Date;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Base<PE: PricingEngine> {
    lazy: LazyObject,
    npv: Money,
    error_estimate: Money,
    valuation_date: Date,
    additional_results: HashMap<String, Money>,
    engine: PE,
    has_engine: bool,
}

impl<PE> Instrument for Base<PE>
where
    PE: PricingEngine,
{
    type E = PE;
    /// returns the net present value of the instrument.
    fn npv(&mut self) -> Money {
        self.calculate();
        assert!(self.npv != Money::default());
        self.npv
    }
    /// returns the error estimate on the NPV when available.
    fn error_estimate(&mut self) -> Money {
        self.calculate();
        assert!(self.error_estimate != Money::default());
        self.error_estimate
    }
    /// returns the date the net present value refers to.
    fn valuation_date(&mut self) -> Date {
        self.calculate();
        assert!(self.valuation_date != Date::default());
        self.valuation_date
    }
    /// returns any additional result returned by the pricing engine.
    fn result(&mut self, tag: String) -> Result<Money, &str> {
        self.calculate();
        let m = self.additional_results.get(&tag);
        if m.is_none() {
            return Err("not found");
        }
        Ok(*m.unwrap())
    }
    /// returns any additional result returned by the pricing engine.
    fn additional_results(&self) -> &HashMap<String, Money> {
        &self.additional_results
    }
    /// returns whether the instrument might have value greater than zero.
    fn is_expired(&self) -> bool {
        false
    }
    /// set the pricing engine to be used.
    fn set_pricing_engine(&mut self, engine: Self::E) {
        if self.has_engine {
            // TODO: unregister.
            self.engine = engine;
            self.has_engine = true;
        }
        if self.has_engine {
            // TODO: register.
            //registerWith(engine_);
        }
        // trigger (lazy) recalculation and notify observers
        self.lazy.update();
    }
    /// When a derived argument structure is defined for an
    /// instrument, this method should be overridden to fill
    /// it. This is mandatory in case a pricing engine is used.
    fn setup_arguments<A: Arguments>(&self, _args: A) {
        unimplemented!();
    }
    /// When a derived result structure is defined for an
    /// instrument, this method should be overridden to read from
    /// it. This is mandatory in case a pricing engine is used.
    fn fetch_results<R: Results>(&mut self, results: R) {
        let r = results.get();
        self.npv = r.value;
        self.error_estimate = r.error_estimate;
        self.valuation_date = r.valuation_date;
        self.additional_results = r.additional_results.clone();
    }

    fn calculate(&mut self) {
        if !self.lazy.calculated {
            if self.is_expired() {
                self.setup_expired();
                self.lazy.calculated = true;
            } else {
                self.lazy.calculate();
            }
        }
    }
    ///
    fn setup_expired(&mut self) {
        self.npv = Money::default();
        self.error_estimate = Money::default();
        self.valuation_date = Date::default();
        self.additional_results.clear();
    }
    ///
    fn perform_calculations(&mut self) {
        assert!(self.has_engine);
        self.engine.reset();
        self.setup_arguments(self.engine.get_arguments());
        self.engine.get_arguments().validate();
        self.engine.calculate();
        self.fetch_results(self.engine.get_results());
    }
}
