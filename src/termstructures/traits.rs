use super::compounding::Compounding;
use super::interestrate::InterestRate;
use crate::definitions::{DiscountFactor, Time};
use crate::time::traits::Calendar as Cal;
use crate::time::Calendar;
use crate::time::Date;
use crate::time::DayCounter;
use crate::time::Frequency;
/// `TermStructure` describes the behavior of a simple term structure.
///
///
pub trait TermStructure {
    /// The latest date for which the curve can return values.
    fn max_date(&self) -> Date;

    /// The settlement days used for reference date calculation.
    fn settlement_days(&self) -> i64;

    /// This method performs a date to double conversion which represents
    /// the fraction of the year between the reference date and the date passed as parameter.
    fn time_from_reference(&self, date: Date) -> Time;

    /// The latest double for which the curve can return values.
    fn max_time(&self) -> Time;

    /// The date at which discount = 1.0 and/or variance = 0.0.
    fn reference_date(&mut self) -> Date;
}

pub trait YieldTermStructure: TermStructure {
    type D: DayCounter;
    /// Returns the discount factor for a given date or time. In the
    /// latter case, the double is calculated as a fraction of year from the
    /// reference date.
    fn discount(&self, date: Date, extrapolate: bool) -> DiscountFactor;
    ///
    fn discount_with_time(&self, time: Time, extrapolate: bool) -> DiscountFactor;

    /// These methods return the implied zero-yield rate for a given date or time.
    /// In the latter case, the time is calculated as a fraction of year from the
    /// reference date.
    fn zero_rate(
        &mut self,
        date: Date,
        result_day_counter: Self::D,
        comp: Compounding,
        freq: Frequency,
        extrapolate: bool,
    ) -> InterestRate<Self::D>;
    ///
    fn zero_rate_with_time(
        &mut self,
        time: Time,
        comp: Compounding,
        freq: Frequency,
        extrapolate: bool,
    ) -> InterestRate<Self::D>;

    /// These methods returns the forward interest rate between two dates or times.
    /// In the latter case, times are calculated as fractions of year from the
    /// reference date.
    /// If both dates (times) are equal the instantaneous forward rate is returned.
    fn forward_rate(
        &mut self,
        d1: Date,
        d2: Date,
        result_day_counter: Self::D,
        comp: Compounding,
        freq: Frequency,
        extrapolate: bool,
    ) -> InterestRate<Self::D>;

    fn forward_rate_with_time(
        &mut self,
        t1: Time,
        t2: Time,
        result_day_counter: Self::D,
        comp: Compounding,
        freq: Frequency,
        extrapolate: bool,
    ) -> InterestRate<Self::D>;
}
