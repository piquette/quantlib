extern crate chrono;
extern crate quantlib;

use crate::quantlib::DayCounter;
use quantlib::time::{Actual360, Date};

#[test]
fn test_actual_360() {
    let start = Date {
        d: chrono::Utc::today(),
    };
    let end = Date {
        d: chrono::Utc::today()
            .checked_add_signed(chrono::Duration::days(4))
            .unwrap(),
    };
    let dc = Actual360 {};

    assert_eq!(dc.day_count(start, end), 4);
}
