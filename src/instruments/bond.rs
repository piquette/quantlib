use super::base::Base;
use super::traits::Instrument;
use crate::cashflows::{CashFlow, Leg};
use crate::definitions::Money;
use crate::pricingengines::bondfunctions;
use crate::pricingengines::{Arguments, PricingEngine, Results};
use crate::time::date as df;
use crate::time::traits::Calendar as Cal;
use crate::time::Calendar;
use crate::time::Date;
use crate::time::TimeUnit;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Bond<C, CF, PE>
where
    C: Cal,
    CF: CashFlow,
    PE: PricingEngine,
{
    pub settlement_days: i64,
    pub calendar: Calendar<C>,
    pub cashflows: Leg<CF>,
    pub issue_date: Date,
    // always computed
    pub redemptions: Leg<CF>,
    pub notionals: Vec<f64>,

    notional_schedule: Vec<Date>,
    maturity_date: Option<Date>,

    // not set in constructor
    settlement_value: Option<f64>,
    base: Base<PE>,
}

impl<C, CF, PE> Bond<C, CF, PE>
where
    C: Cal,
    CF: CashFlow,
    PE: PricingEngine + Default,
{
    ///
    pub fn new(
        settlement_days: i64,
        calendar: Calendar<C>,
        coupons: Leg<CF>,
        issue_date: Date,
    ) -> Bond<C, CF, PE> {
        // build.
        let mut b = Bond {
            settlement_days: settlement_days,
            calendar: calendar,
            cashflows: coupons,
            issue_date: issue_date,
            notionals: vec![],
            notional_schedule: vec![],
            redemptions: vec![],
            maturity_date: None,
            settlement_value: None,
            base: Base::default(),
        };

        if !b.cashflows.is_empty() {
            // TODO: sort cashflows by date.
            // coupons.sort(by earlier than cashflow comparator)
            b.maturity_date = Some(b.cashflows.last().unwrap().date());
            b.add_redemptions_to_cashflows();
        }

        // TODO: add observer.
        return b;
    }

    ///
    pub fn new_today(settlement_days: i64, calendar: Calendar<C>) -> Bond<C, CF, PE> {
        Bond::new(settlement_days, calendar, vec![], Date::default())
    }

    ///
    pub fn new_with_issue_date(
        settlement_days: i64,
        calendar: Calendar<C>,
        issue_date: Date,
    ) -> Bond<C, CF, PE> {
        Bond::new(settlement_days, calendar, vec![], issue_date)
    }

    ///
    pub fn new_non_amortizing(
        settlement_days: i64,
        calendar: Calendar<C>,
        face_amount: f64,
        maturity_date: Date,
        cashflows: Leg<CF>,
        issue_date: Date,
    ) -> Bond<C, CF, PE> {
        // build.
        let mut b = Bond {
            settlement_days: settlement_days,
            calendar: calendar,
            cashflows: cashflows,
            issue_date: issue_date,
            notionals: vec![],
            notional_schedule: vec![],
            redemptions: vec![],
            maturity_date: Some(maturity_date),
            settlement_value: None,
            base: Base::default(),
        };

        if !b.cashflows.is_empty() {
            b.notional_schedule.push(Date::default());
            b.notionals.push(face_amount);
            b.notional_schedule.push(maturity_date);
            b.notionals.push(0.0);
            unimplemented!();
            //
            // let last = b.cashflows.pop().unwrap();
            // let cp = last;
            // b.redemptions.push(last);

            // // TODO: sort cashflows except last one, by date.
            // // coupons.sort(by earlier than cashflow comparator)
            // //
            // // then add it back.
            // b.cashflows.push(cp);
        }
        // TODO: add observer.

        return b;
    }

    ///
    pub fn new_non_amortizing_today(
        settlement_days: i64,
        calendar: Calendar<C>,
        face_amount: f64,
        maturity_date: Date,
    ) -> Bond<C, CF, PE> {
        Bond::new_non_amortizing(
            settlement_days,
            calendar,
            face_amount,
            maturity_date,
            vec![],
            Date::default(),
        )
    }

    ///
    pub fn new_non_amortizing_with_issue_date(
        settlement_days: i64,
        calendar: Calendar<C>,
        face_amount: f64,
        maturity_date: Date,
        issue_date: Date,
    ) -> Bond<C, CF, PE> {
        Bond::new_non_amortizing(
            settlement_days,
            calendar,
            face_amount,
            maturity_date,
            vec![],
            issue_date,
        )
    }

    //
    // Getters.
    //
    pub fn face_amount(&self) -> Option<&f64> {
        self.notionals.get(0)
    }

    ///
    pub fn settlement_date(&self, d: Option<Date>) -> Date {
        let mut date = Date::default();
        if d.is_none() {
            // TODO: should be global eval date.
            date = Date::default()
        }

        // usually, the settlement is at T+n...
        let settlement =
            self.calendar
                .advance_by_units(date, self.settlement_days as usize, TimeUnit::Days);
        // ...but the bond won't be traded until the issue date (if given.)
        if self.issue_date == Date::default() {
            return settlement;
        } else {
            df::max(settlement, self.issue_date)
        }
    }

    ///
    pub fn notional(&self, date: Option<Date>) -> f64 {
        let d = if date.is_none() {
            self.settlement_date(date)
        } else {
            date.unwrap()
        };

        if d > *self.notional_schedule.last().unwrap() {
            // after maturity
            return 0.0;
        }

        // After the check above, d is between the schedule
        // boundaries.  We search starting from the second notional
        // date, since the first is null.  After the call to
        // lower_bound, *i is the earliest date which is greater or
        // equal than d.  Its index is greater or equal to 1.
        let mut idx = 0;
        for nd in &self.notional_schedule {
            if nd >= &d {
                break;
            }
            idx = idx + 1;
        }
        assert!(idx != 0);
        if d < self.notional_schedule[idx] {
            // no doubt about what to return
            return self.notionals[idx - 1];
        } else {
            // d is equal to a redemption date.
            // As per bond conventions, the payment has occurred;
            // the bond already changed notional.
            return self.notionals[idx];
        }
    }

    ///
    pub fn redemption(&self) -> &CF {
        assert!(self.redemptions.len() == 1);
        self.redemptions.last().unwrap()
    }

    ///
    pub fn start_date(self) -> Date {
        bondfunctions::start_date(self)
    }

    ///
    pub fn maturity_date(self) -> Date {
        if self.maturity_date.is_some() {
            return self.maturity_date.unwrap();
        }
        bondfunctions::maturity_date(self)
    }

    ///
    pub fn is_tradeable(self, d: Date) -> bool {
        bondfunctions::is_tradeable(self, d)
    }

    ///
    fn add_redemptions_to_cashflows(&self) {}

    // Calculations.
    // ==============

    ///
    pub fn clean_price(&self) -> f64 {
        self.dirty_price() - self.accrued_amount(self.settlement_date(None))
    }

    ///
    pub fn dirty_price(&self) -> f64 {
        let current_notional = self.notional(Some(self.settlement_date(None)));
        if current_notional == 0.0 {
            return 0.0;
        }
        self.settlement_value() * 100.0 / current_notional
    }

    ///
    pub fn settlement_value(&self) -> f64 {
        0.0
    }

    ///
    pub fn settlement_value_from_clean(&self, clean_price: f64) -> f64 {
        0.0
    }

    ///
    pub fn accrued_amount(&self, settlement: Date) -> f64 {
        0.0
    }
}

impl<C, CF, PE> Instrument for Bond<C, CF, PE>
where
    C: Cal,
    CF: CashFlow,
    PE: PricingEngine,
{
    type E = PE;
    /// returns the net present value of the instrument.
    fn npv(&mut self) -> Money {
        self.base.npv()
    }
    /// returns the error estimate on the NPV when available.
    fn error_estimate(&mut self) -> Money {
        self.base.error_estimate()
    }
    /// returns the date the net present value refers to.
    fn valuation_date(&mut self) -> Date {
        self.base.valuation_date()
    }
    /// returns any additional result returned by the pricing engine.
    fn result(&mut self, tag: String) -> Result<Money, &str> {
        self.base.result(tag)
    }
    /// returns any additional result returned by the pricing engine.
    fn additional_results(&self) -> &HashMap<String, Money> {
        self.base.additional_results()
    }
    /// returns whether the instrument might have value greater than zero.
    fn is_expired(&self) -> bool {
        unimplemented!();
        //TODO: make sure this is implemented.
        false
    }
    /// set the pricing engine to be used.
    fn set_pricing_engine(&mut self, engine: Self::E) {
        self.base.set_pricing_engine(engine)
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
        self.base.fetch_results(results)
    }

    fn calculate(&mut self) {
        self.base.calculate()
    }
    ///
    fn setup_expired(&mut self) {
        self.base.setup_expired()
    }
    ///
    fn perform_calculations(&mut self) {
        self.base.perform_calculations()
    }
}
