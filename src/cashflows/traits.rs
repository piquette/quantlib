use crate::definitions::Time;
use crate::time::{Date, DayCounter};

pub trait Event {
    fn date(&self) -> Date;
    fn has_occured(&self, date: Date) -> bool;
}

pub trait CashFlow: Event {
    fn amount(&self) -> f64 {
        0.0
    }
    fn try_as_coup(&self) -> Option<&dyn Coupon> {
        None
    }
    fn has_occured(&self, date: Date, include_today: bool) -> bool;
    fn ex_coupon_date(&self) -> Date {
        Date::default()
    }
    fn trading_ex_coupon(&self) -> bool;
}

pub trait Coupon: CashFlow {
    fn nominal(&self) -> f64;
    /// start of the accrual period
    fn accrual_start_date(&self) -> Date;
    /// end of the accrual period
    fn accrual_end_date(&self) -> Date;
    /// start date of the reference period
    fn reference_period_start(&self) -> Date;
    /// end date of the reference period
    fn reference_period_end(&self) -> Date;
    /// accrual period as fraction of year
    fn accrual_period(&self) -> Time;
    /// accrual period in days
    fn accrual_days(&self) -> i64;
    /// accrued rate
    fn rate(&self) -> f64;
    // day counter for accrual calculation
    // fn day_counter(&self) -> Option<&dyn DayCounter>;
    /// accrued period as fraction of year at the given date
    fn accrued_period(&self) -> Time;
    /// accrued days at the given date
    fn accrued_days(&self) -> i64;
    /// accrued amount at the given date
    fn accrued_amount(&self, _date: Date) -> f64;
}
