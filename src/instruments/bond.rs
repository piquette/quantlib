pub use crate::cashflows::Leg;
pub use crate::time::Calendar;
pub use crate::time::Date;

pub struct Bond {
    pub settlement_days: i32,
    pub calendar: Calendar,
    pub notional_schedule: Vec<Date>,
    pub notionals: Vec<f64>,
    pub cashflows: Leg,
    pub redemptions: Leg,
    pub maturity_date: Date,
    pub issue_date: Date,
    pub settlement_value: f64,
}
