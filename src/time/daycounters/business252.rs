use crate::time::traits::Calendar as Cal;
use crate::time::traits::DayCounter;
use crate::time::{Calendar, Date};

#[derive(Copy, Clone)]
pub struct Business252<C: Cal> {
    pub calendar: Calendar<C>,
}

//
// Business/252 day count convention.
// http://en.wikipedia.org/wiki/Day_count_convention
//
impl<C: Cal> DayCounter for Business252<C> {
    //
    //
    fn day_count(&self, date_start: Date, date_end: Date) -> i64 {
        self.calendar.business_days_between(date_start, date_end)
    }

    //
    //
    fn year_fraction(
        &self,
        date_start: Date,
        date_end: Date,
        _ref_period_start: Option<Date>,
        _ref_period_end: Option<Date>,
    ) -> f64 {
        self.day_count(date_start, date_end) as f64 / 252.0
    }
}
