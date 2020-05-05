use super::day_count;
use crate::time::traits::*;
use crate::time::Date;

pub struct Actual360;

//
// Actual/360 day count convention, also known as "Act/360", or "A/360".
// http://en.wikipedia.org/wiki/Day_count_convention
//
impl DayCounter for Actual360 {
    //
    //
    fn day_count(&self, date_start: Date, date_end: Date) -> i64 {
        day_count(date_start, date_end)
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
        self.day_count(date_start, date_end) as f64 / 360.0
    }
}
