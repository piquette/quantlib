use crate::time::Date;

pub trait DayCounter {
    fn day_count(&self, date_start: Date, date_end: Date) -> i64;
    fn year_fraction(
        &self,
        date_start: Date,
        date_end: Date,
        ref_period_start: Date,
        ref_period_end: Date,
    ) -> f64;
}

pub struct DayCounterBase;

impl DayCounter for DayCounterBase {
    fn day_count(&self, _date_start: Date, _date_end: Date) -> i64 {
        0
    }
    fn year_fraction(
        &self,
        _date_start: Date,
        _date_end: Date,
        _ref_period_start: Date,
        _ref_period_end: Date,
    ) -> f64 {
        0.0
    }
}
