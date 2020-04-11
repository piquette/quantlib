use super::Compounding;
use crate::daycounters::{DayCounter, DayCounterBase};
pub struct InterestRate {
    pub rate: f64,
    pub day_counter: Box<dyn DayCounter>,
    pub compounding: Compounding,
    pub freq_makes_sense: bool,
    pub freq: i32,
}
