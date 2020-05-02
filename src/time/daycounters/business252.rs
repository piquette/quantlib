use crate::time::traits::*;
use crate::time::{Calendar, Date};

pub struct Business252 {
    pub calendar: Calendar,
}

//
// Business/252 day count convention.
// http://en.wikipedia.org/wiki/Day_count_convention
//
impl DayCounter for Business252 {
    //
    //
    fn day_count(&self, date_start: Date, date_end: Date) -> usize {
        self.calendar.business_days_between(date_start, date_end) as usize
    }

    //
    //
    fn year_fraction(
        &self,
        date_start: Date,
        date_end: Date,
        _ref_period_start: Date,
        _ref_period_end: Date,
    ) -> f64 {
        self.day_count(date_start, date_end) as f64 / 252.0
    }
}
