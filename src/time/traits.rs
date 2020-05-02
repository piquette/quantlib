use super::date::Date;
use super::weekday::Weekday;

pub trait DayCounter {
    fn day_count(&self, date_start: Date, date_end: Date) -> usize;
    fn year_fraction(
        &self,
        date_start: Date,
        date_end: Date,
        ref_period_start: Date,
        ref_period_end: Date,
    ) -> f64;
}

pub trait Calendar {
    fn name(&self) -> String;
    fn is_business_day(&self, date: Date) -> bool;
    fn is_weekend(&self, weekday: &Weekday) -> bool;
}
