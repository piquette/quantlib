use super::date::Date;
use super::weekday::Weekday;
use crate::definitions::Time;

pub trait DayCounter {
    fn day_count(&self, date_start: Date, date_end: Date) -> i64;
    fn year_fraction(
        &self,
        date_start: Date,
        date_end: Date,
        ref_period_start: Option<Date>,
        ref_period_end: Option<Date>,
    ) -> Time;
}

pub trait Calendar {
    fn name(&self) -> String;
    fn is_business_day(&self, date: Date) -> bool;
    fn is_weekend(&self, weekday: &Weekday) -> bool;
}
