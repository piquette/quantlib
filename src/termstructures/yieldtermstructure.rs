use super::base::Base;
use super::interestrate::InterestRate;
use super::traits::TermStructure;
use super::traits::YieldTermStructure as YTS;
use crate::definitions::{DiscountFactor, Time};
use crate::quotes::Quote;
use crate::time::{Calendar, Date, DayCounter, Month};

type DiscountImpl = Box<dyn Fn(Time) -> DiscountFactor>;

pub struct YieldTermStructure {
    base: Base,
    jumps: Vec<Box<dyn Quote>>,
    jump_times: Vec<Time>,
    jump_dates: Vec<Date>,
    latest_reference: Option<Date>,
    jumps_num: usize,
    discount_impl: Option<DiscountImpl>,
}

impl Default for YieldTermStructure {
    fn default() -> YieldTermStructure {
        YieldTermStructure {
            base: Base::default(),
            jumps: vec![],
            jump_times: vec![],
            jump_dates: vec![],
            jumps_num: 0,
            latest_reference: None,
            discount_impl: None,
        }
    }
}

impl YieldTermStructure {
    pub fn new(
        &self,
        calendar: Calendar,
        reference_date: Date,
        day_counter: Box<dyn DayCounter>,
        settlement_days: i64,
        jumps: Vec<Box<dyn Quote>>,
        jump_dates: Vec<Date>,
        discount_impl: DiscountImpl,
    ) -> YieldTermStructure {
        // Set fields.
        let yt = Self::default();
        yt.base.calendar = Some(calendar);
        yt.base.reference_date = Some(reference_date);
        yt.base.day_counter = day_counter;
        yt.base.settlement_days = settlement_days;
        yt.discount_impl = Some(discount_impl);
        // Set jumps
        yt.jumps = jumps;
        yt.jump_dates = jump_dates;
        yt.jumps_num = jumps.len();
        yt.set_jumps();
        yt
    }

    /// Set jumps.
    fn set_jumps(&self) {
        if self.jump_dates.is_empty() && !self.jumps.is_empty() {
            //
            self.jump_times.resize_with(self.jumps_num, || 0.0);
            self.jump_dates
                .resize_with(self.jumps_num, || Date::default());
            let y = self.reference_date().year();
            for n in 0..=self.jumps_num {
                self.jump_dates[n] = Date::new(31, Month::December, (y + n) as i32);
            }
        }
        for n in 0..=self.jumps_num {
            self.jump_times[n] = self.time_from_reference(self.jump_dates[n]);
        }
        self.latest_reference = Some(self.reference_date());
    }

    pub fn set_calendar(&self, calendar: Calendar) {
        self.base.calendar = Some(calendar)
    }
    pub fn set_reference_date(&self, date: Date) {
        self.base.reference_date = Some(date)
    }
    pub fn set_day_counter(&self, day_counter: Box<dyn DayCounter>) {
        self.base.day_counter = day_counter
    }
    pub fn set_settlement_days(&self, settlement_days: i64) {
        self.base.settlement_days = settlement_days;
    }
}

impl YTS for YieldTermStructure {
    /// Returns the discount factor for a given date or time. In the
    /// latter case, the double is calculated as a fraction of year from the
    /// reference date.
    fn discount(&self, date: Date, extrapolate: bool) -> DiscountFactor {
        self.discount_with_time(self.time_from_reference(date), extrapolate)
    }
    ///
    fn discount_with_time(&self, time: Time, extrapolate: bool) -> DiscountFactor {
        //
        self.base
            .check_range_with_time(time, self.max_time(), extrapolate);
        //
        if self.jumps.is_empty() {
            return self.discount_impl.unwrap()(time);
        }

        let jump_effect: DiscountFactor = 1.0;
        for n in 0..=self.jumps_num {
            if self.jump_times[n] > 0.0 && self.jump_times[n] < time {
                assert!(self.jumps[n].is_valid());
                let this_jump = self.jumps[n].value();
                assert!(this_jump > 0.0);
                jump_effect *= this_jump;
            }
        }

        jump_effect * self.discount_impl.unwrap()(time)
    }

    /// These methods return the implied zero-yield rate for a given date or time.
    /// In the latter case, the time is calculated as a fraction of year from the
    /// reference date.
    fn zero_rate(
        &self,
        date: Date,
        result_day_counter: Box<dyn DayCounter>,
        comp: Compounding,
        freq: Frequency,
        extrapolate: bool,
    ) -> InterestRate;
    ///
    fn zero_rate_with_time(
        &self,
        time: Time,
        comp: Compounding,
        freq: Frequency,
        extrapolate: bool,
    ) -> InterestRate;

    /// These methods returns the forward interest rate between two dates or times.
    /// In the latter case, times are calculated as fractions of year from the
    /// reference date.
    /// If both dates (times) are equal the instantaneous forward rate is returned.
    fn forward_rate(
        &self,
        d1: Date,
        d2: Date,
        result_day_counter: Box<dyn DayCounter>,
        comp: Compounding,
        freq: Frequency,
        extrapolate: bool,
    ) -> InterestRate;

    fn forward_rate_with_period(
        &self,
        d: Date,
        p: Period,
        result_day_counter: Box<dyn DayCounter>,
        comp: Compounding,
        freq: Frequency,
        extrapolate: bool,
    ) -> InterestRate;

    fn forward_rate_with_time(
        &self,
        t1: Time,
        t2: Time,
        result_day_counter: Box<dyn DayCounter>,
        comp: Compounding,
        freq: Frequency,
        extrapolate: bool,
    ) -> InterestRate;
}

impl TermStructure for YieldTermStructure {
    /// The latest date for which the curve can return values.
    fn max_date(&self) -> Date {
        self.base.max_date()
    }

    /// The calendar used for reference date calculation.
    fn calendar(&self) -> Calendar {
        self.base.calendar()
    }

    /// The settlement days used for reference date calculation.
    fn settlement_days(&self) -> i64 {
        self.base.settlement_days()
    }

    /// This method performs a date to double conversion which represents
    /// the fraction of the year between the reference date and the date passed as parameter.
    fn time_from_reference(&self, date: Date) -> Time {
        self.base.time_from_reference(date)
    }

    /// The day counter used for date/double conversion.
    fn day_counter(&self) -> Box<dyn DayCounter> {
        self.base.day_counter()
    }

    /// The latest double for which the curve can return values.
    fn max_time(&self) -> Time {
        self.base.max_time()
    }

    /// The date at which discount = 1.0 and/or variance = 0.0.
    fn reference_date(&self) -> Date {
        self.base.reference_date()
    }
}
