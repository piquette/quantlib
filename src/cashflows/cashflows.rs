use super::traits::CashFlow;
use super::Leg;

use crate::time::date as df;
use crate::time::Date;

pub fn start_date<CF: CashFlow>(leg: &Leg<CF>) -> Date {
    assert!(!leg.is_empty());
    //
    let mut d = df::MAX_DATE;
    for c in leg {
        match c.try_as_coup() {
            Some(coup) => {
                d = df::min(d, coup.accrual_start_date());
            }
            None => {
                d = df::min(d, c.date());
            }
        }
    }
    //
    d
}

pub fn maturity_date<CF: CashFlow>(leg: &Leg<CF>) -> Date {
    assert!(!leg.is_empty());
    //
    let mut d = df::MIN_DATE;
    for c in leg {
        match c.try_as_coup() {
            Some(coup) => {
                d = df::max(d, coup.accrual_start_date());
            }
            None => {
                d = df::max(d, c.date());
            }
        }
    }
    //
    d
}
