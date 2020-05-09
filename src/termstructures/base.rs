use super::traits::TermStructure;
use crate::definitions::Time;
use crate::time::traits::Calendar as Cal;
use crate::time::Actual365Fixed;
use crate::time::Calendar;
use crate::time::Date;
use crate::time::DayCounter;
use crate::time::TimeUnit;

pub struct Base<C: Cal, DC = Actual365Fixed> {
    pub settlement_days: i64,
    pub day_counter: DC,
    pub moving: bool,
    pub updated: bool,
    pub calendar: Option<Calendar<C>>,
    pub reference_date: Option<Date>,
}

//impl<DC: DayCounter> Default for Base<DC> {}

impl<C, DC> Base<C, DC>
where
    C: Cal,
    DC: DayCounter,
{
    pub fn default() -> Base<C, Actual365Fixed> {
        Base {
            moving: false,
            updated: true,
            settlement_days: 0,
            day_counter: Actual365Fixed {},
            calendar: None,
            reference_date: None,
        }
    }

    pub fn new(day_counter: DC) -> Base<C, DC> {
        Base {
            moving: false,
            updated: true,
            settlement_days: 0,
            day_counter: day_counter,
            calendar: None,
            reference_date: None,
        }
    }

    pub fn check_range(&self, d: Date, ref_date: Date, max: Date, extrapolate: bool) {
        assert!(d >= ref_date);
        assert!(d <= max);
    }
    pub fn check_range_with_time(&self, t: Time, max: Time, extrapolate: bool) {
        assert!(t >= 0.0);
        assert!(t <= max);
    }
}

impl<C: Cal, DC: DayCounter> TermStructure for Base<C, DC> {
    /// The latest date for which the curve can return values.
    fn max_date(&self) -> Date {
        Date::default()
    }

    /// The settlement days used for reference date calculation.
    fn settlement_days(&self) -> i64 {
        self.settlement_days
    }

    /// This method performs a date to double conversion which represents
    /// the fraction of the year between the reference date and the date passed as parameter.
    fn time_from_reference(&self, date: Date) -> Time {
        self.day_counter
            .year_fraction(self.reference_date.unwrap(), date, None, None)
    }

    /// The latest double for which the curve can return values.
    fn max_time(&self) -> Time {
        self.time_from_reference(self.max_date())
    }

    /// The date at which discount = 1.0 and/or variance = 0.0.
    fn reference_date(&mut self) -> Date {
        if !self.updated {
            self.reference_date = Some(self.calendar.unwrap().advance_by_units(
                Date::default(),
                self.settlement_days as usize,
                TimeUnit::Days,
            ));
            self.updated = true;
        }
        self.reference_date.unwrap()
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
