use crate::time::traits::*;
use crate::time::Date;

pub fn day_count(date_start: Date, date_end: Date) -> usize {
    date_end.sub(date_start)
}

pub struct Simple;

impl DayCounter for Simple {
    fn day_count(&self, date_start: Date, date_end: Date) -> usize {
        //
        // Supposed to implement whatever Thirty360 implements.
        //
        day_count(date_start, date_end)
    }
    fn year_fraction(
        &self,
        date_start: Date,
        date_end: Date,
        _ref_period_start: Date,
        _ref_period_end: Date,
    ) -> f64 {
        let dm1 = date_start.day_of_month();
        let dm2 = date_end.day_of_month();
        let m1 = date_start.month() as usize;
        let m2 = date_end.month() as usize;
        let y1 = date_start.year();
        let y2 = date_end.year();

        if dm1 == dm2
            || dm1 > dm2 && Date::is_end_of_month(date_end)
            || dm1 < dm2 && Date::is_end_of_month(date_start)
        {
            ((y2 - y1) + (m2 - m1) as usize) as f64 / 12.0
        } else {
            // fallback to Thirty360
            0.0
        }
    }
}
