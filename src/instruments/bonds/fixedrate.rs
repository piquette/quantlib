use super::super::bond::Bond;
use crate::time::{Calendar, Frequency};

pub struct FixedRateBond {
    pub bond: Bond,
    pub frequency: Frequency,
    pub calendar: Calendar,
}
