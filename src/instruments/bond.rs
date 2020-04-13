pub use crate::cashflows::Leg;
pub use crate::time::Calendar;
pub use crate::time::Date;

pub struct Bond {
    pub settlement_days: i32,
    pub calendar: Calendar,
    pub cashflows: Leg,
    pub maturity_date: Date,
    pub issue_date: Date,
    // always computed
    redemptions: Leg,
    notional_schedule: Vec<Date>,
    notionals: Vec<f64>,
    // not set in constructor
    settlement_value: f64,
}
// constructor has:
// settlement days int
// calendar
// face amt double
// maturity date
// issue date
// cash flows leg
