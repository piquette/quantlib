use super::day_count;
use crate::time::traits::*;
use crate::time::Date;

#[derive(Copy, Clone)]
pub struct Actual365Fixed;

//
// "Actual/365 (Fixed)" day count convention, also know as
// "Act/365 (Fixed)", "A/365 (Fixed)", or "A/365F".
// According to ISDA, "Actual/365" (without "Fixed") is
// an alias for "Actual/Actual (ISDA)"DayCounter (see
// ActualActual.)  If Actual/365 is not explicitly
// specified as fixed in an instrument specification,
// you might want to double-check its meaning.
//
// http://en.wikipedia.org/wiki/Day_count_convention
//
impl DayCounter for Actual365Fixed {
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
        self.day_count(date_start, date_end) as f64 / 365.0
    }
}
