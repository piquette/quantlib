use crate::time::Calendar;
use crate::time::Date;
use crate::time::DayCounter;

pub struct Base {
    settlement_days: usize,
    day_counter: Box<dyn DayCounter>,
    moving: bool,
    updated: bool,
    calendar: Option<Calendar>,
    reference_date: Option<Date>,
}

impl Base {
    pub fn new(day_counter: Box<dyn DayCounter>) -> Base {
        Base {
            moving: false,
            updated: true,
            settlement_days: 0,
            day_counter: day_counter,
            calendar: None,
            reference_date: None,
        }
    }
}

// Three ways to keep track of the reference date.
// 1. Fixed
// 2. Advanced from today by a set number of business days.
// 3. Determined by another structure and overriden.
// explicit TermStructure(const DayCounter& dc = DayCounter());
// //! initialize with a fixed reference date
// explicit TermStructure(const Date& referenceDate,
//                         const Calendar& calendar = Calendar(),
//                         const DayCounter& dc = DayCounter());
// //! calculate the reference date based on the global evaluation date
// TermStructure(Natural settlementDays,
//                 const Calendar&,
//                 const DayCounter& dc = DayCounter());
