use crate::time::Date;
use crate::time::DayCounter;

#[derive(Copy, Clone)]
pub struct Base<DC: DayCounter> {
    pub nominal: f64,
    pub day_counter: DC,
    pub payment_date: Date,
    pub accrual_start_date: Date,
    pub accrual_end_date: Date,
    pub reference_period_start: Date,
    pub reference_period_end: Date,
}
