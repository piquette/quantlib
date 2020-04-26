use super::super::bond::Bond;
use crate::time::{DayCounter, Frequency};

pub struct FixedRateBond {
    pub bond: Bond,
    pub frequency: Frequency,
    pub day_counter: DayCounter,
}
