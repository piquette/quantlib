use crate::time::Date;

pub trait Event: Copy {
    fn date(&self) -> Date;
    fn has_occured(&self, date: Date, include_today: bool) -> bool;
}

pub trait CashFlow: Event {
    fn amount(&self) -> f64;
}
