use crate::time::Calendar;
use crate::time::Date;
use crate::time::DayCounter;

/// `TermStructure` describes the behavior of a simple term structure.
///
///
pub trait TermStructure {
    /// The latest date for which the curve can return values.
    fn max_date() -> Date;

    /// The calendar used for reference date calculation.
    fn calendar() -> Calendar;

    /// The settlement days used for reference date calculation.
    fn settlement_days() -> usize;

    /// This method performs a date to double conversion which represents
    /// the fraction of the year between the reference date and the date passed as parameter.
    fn time_from_reference(date: Date) -> f64;

    /// The day counter used for date/double conversion.
    fn day_counter() -> Box<dyn DayCounter>;

    /// The latest double for which the curve can return values.
    fn max_time() -> f64;

    /// The date at which discount = 1.0 and/or variance = 0.0.
    fn reference_date() -> Date;
}

pub trait YieldTermStructure {}
