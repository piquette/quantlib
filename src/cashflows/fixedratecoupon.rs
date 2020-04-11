use super::{CashFlow, Coupon, CouponFields, Event};
use crate::termstructures::InterestRate;
use crate::time::Date;

pub struct FixedRateCoupon {
    pub fields: CouponFields,
    pub interest_rate: InterestRate,
}

impl Coupon for FixedRateCoupon {
    fn rate(&self) -> f64 {
        self.interest_rate.rate
    }
    fn accrued_amount(&self, _date: Date) -> f64 {
        0.0
    }
    fn accrual_period(&self) -> f64 {
        self.fields.day_counter.year_fraction(
            self.fields.accrual_start_date,
            self.fields.accrual_end_date,
            self.fields.reference_period_start,
            self.fields.reference_period_end,
        )
    }
    fn accrual_days(&self) -> i64 {
        self.fields
            .day_counter
            .day_count(self.fields.accrual_start_date, self.fields.accrual_end_date)
    }
}
impl CashFlow for FixedRateCoupon {
    fn amount(&self) -> f64 {
        0.0
    }
}

impl Event for FixedRateCoupon {
    fn date(&self) -> Date {
        self.fields.payment_date
    }
    fn has_occured(&self, date: Date, include_today: bool) -> bool {
        if include_today {
            self.fields.payment_date.d.le(&date.d)
        } else {
            self.fields.payment_date.d.le(&date.d) || self.fields.payment_date.d.eq(&date.d)
        }
    }
}
