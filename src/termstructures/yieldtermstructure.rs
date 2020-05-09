use super::base::Base;
use super::compounding::Compounding;
use super::interestrate::InterestRate;
use super::traits::TermStructure;
use super::traits::YieldTermStructure as YTS;
use crate::definitions::{DiscountFactor, Time};
use crate::quotes::Quote;
use crate::time::traits::Calendar as Cal;
use crate::time::{Actual365Fixed, Calendar, Date, DayCounter, Frequency, Month};

type DiscountImpl = Box<dyn Fn(Time) -> DiscountFactor>;
const DT: Time = 0.0001;

pub struct YieldTermStructure<C: Cal, Q: Quote, DC = Actual365Fixed> {
    base: Base<C, DC>,
    jumps: Vec<Q>,
    jump_times: Vec<Time>,
    jump_dates: Vec<Date>,
    latest_reference: Option<Date>,
    jumps_num: usize,
    discount_impl: Option<DiscountImpl>,
}

// fn default() -> YieldTermStructure<Actual365Fixed, Q> {
//     YieldTermStructure {
//         base: Base::default(),
//         jumps: vec![],
//         jump_times: vec![],
//         jump_dates: vec![],
//         jumps_num: 0,
//         latest_reference: None,
//         discount_impl: None,
//     }
// }

impl<C, Q, DC> YieldTermStructure<C, Q, DC>
where
    C: Cal,
    Q: Quote,
    DC: DayCounter,
{
    pub fn new(
        calendar: Calendar<C>,
        reference_date: Date,
        day_counter: DC,
        settlement_days: i64,
        jumps: Vec<Q>,
        jump_dates: Vec<Date>,
        discount_impl: DiscountImpl,
    ) -> YieldTermStructure<C, Q, DC> {
        // Set fields.
        let mut yt = YieldTermStructure {
            base: Base::new(day_counter),
            jumps: vec![],
            jump_times: vec![],
            jump_dates: vec![],
            jumps_num: 0,
            latest_reference: None,
            discount_impl: None,
        };
        yt.base.calendar = Some(calendar);
        yt.base.reference_date = Some(reference_date);
        yt.base.settlement_days = settlement_days;
        yt.discount_impl = Some(discount_impl);
        // Set jumps
        yt.jumps = jumps;
        yt.jump_dates = jump_dates;
        yt.jumps_num = yt.jumps.len();
        yt.set_jumps();
        yt
    }

    /// Set jumps.
    fn set_jumps(&mut self) {
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

    pub fn set_calendar(&mut self, calendar: Calendar<C>) {
        self.base.calendar = Some(calendar)
    }
    pub fn set_reference_date(&mut self, date: Date) {
        self.base.reference_date = Some(date)
    }
    pub fn set_day_counter(&mut self, day_counter: DC) {
        self.base.day_counter = day_counter
    }
    pub fn set_settlement_days(&mut self, settlement_days: i64) {
        self.base.settlement_days = settlement_days;
    }
}

impl<C, Q, DC> YTS for YieldTermStructure<C, Q, DC>
where
    C: Cal,
    Q: Quote,
    DC: DayCounter,
{
    type D = DC;
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
            return self.discount_impl.as_ref().unwrap()(time);
        }

        let mut jump_effect: DiscountFactor = 1.0;
        for n in 0..=self.jumps_num {
            if self.jump_times[n] > 0.0 && self.jump_times[n] < time {
                assert!(self.jumps[n].is_valid());
                let this_jump = self.jumps[n].value();
                assert!(this_jump > 0.0);
                jump_effect *= this_jump;
            }
        }

        jump_effect * self.discount_impl.as_ref().unwrap()(time)
    }

    /// These methods return the implied zero-yield rate for a given date or time.
    /// In the latter case, the time is calculated as a fraction of year from the
    /// reference date.
    fn zero_rate(
        &mut self,
        date: Date,
        result_day_counter: DC,
        comp: Compounding,
        freq: Frequency,
        extrapolate: bool,
    ) -> InterestRate<DC> {
        if date == self.reference_date() {
            let compound = 1.0 / self.discount_with_time(DT, extrapolate);
            return InterestRate::implied_rate_with_time(
                compound,
                result_day_counter,
                comp,
                freq,
                DT,
            );
        }
        let compound = 1.0 / self.discount(date, extrapolate);
        InterestRate::implied_rate(
            compound,
            result_day_counter,
            comp,
            freq,
            self.reference_date(),
            date,
            None,
            None,
        )
    }
    ///
    fn zero_rate_with_time(
        &mut self,
        time: Time,
        comp: Compounding,
        freq: Frequency,
        extrapolate: bool,
    ) -> InterestRate<DC> {
        let mut t = time;
        if time == 0.0 {
            t = DT;
        }
        let compound = 1.0 / self.discount_with_time(DT, extrapolate);
        return InterestRate::implied_rate_with_time(
            compound,
            self.base.day_counter,
            comp,
            freq,
            t,
        );
    }

    /// These methods returns the forward interest rate between two dates or times.
    /// In the latter case, times are calculated as fractions of year from the
    /// reference date.
    /// If both dates (times) are equal the instantaneous forward rate is returned.
    fn forward_rate(
        &mut self,
        d1: Date,
        d2: Date,
        result_day_counter: DC,
        comp: Compounding,
        freq: Frequency,
        extrapolate: bool,
    ) -> InterestRate<DC> {
        if d1 == d2 {
            let rf = self.reference_date();
            let md = self.max_date();
            self.base.check_range(d1, rf, md, extrapolate);

            let t1 = ((self.time_from_reference(d1) - DT / 2.0) as f64).max(0.0);
            let t2 = t1 + DT;

            let compound = self.discount_with_time(t1, true) / self.discount_with_time(t2, true);
            // times have been calculated with a possibly different daycounter
            // but the difference should not matter for very small times
            return InterestRate::implied_rate_with_time(
                compound,
                result_day_counter,
                comp,
                freq,
                DT,
            );
        }
        assert!(d1 < d2);
        let compound = self.discount(d1, extrapolate) / self.discount(d2, extrapolate);
        InterestRate::implied_rate(compound, result_day_counter, comp, freq, d1, d2, None, None)
    }

    fn forward_rate_with_time(
        &mut self,
        mut t1: Time,
        mut t2: Time,
        result_day_counter: DC,
        comp: Compounding,
        freq: Frequency,
        extrapolate: bool,
    ) -> InterestRate<DC> {
        let compound: f64;
        if t2 == t1 {
            self.base
                .check_range_with_time(t1, self.max_time(), extrapolate);
            t1 = (t1 - DT / 2.0).max(0.0);
            t2 = t1 + DT;
            compound = self.discount_with_time(t1, true) / self.discount_with_time(t2, true);
        } else {
            // QL_REQUIRE(t2 > t1, "t2 (" << t2 << ") < t1 (" << t2 << ")");
            compound =
                self.discount_with_time(t1, extrapolate) / self.discount_with_time(t2, extrapolate);
        }

        InterestRate::implied_rate_with_time(compound, result_day_counter, comp, freq, t2 - t1)
    }
}

impl<C, Q, DC> TermStructure for YieldTermStructure<C, Q, DC>
where
    C: Cal,
    Q: Quote,
    DC: DayCounter,
{
    /// The latest date for which the curve can return values.
    fn max_date(&self) -> Date {
        self.base.max_date()
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

    /// The latest double for which the curve can return values.
    fn max_time(&self) -> Time {
        self.base.max_time()
    }

    /// The date at which discount = 1.0 and/or variance = 0.0.
    fn reference_date(&mut self) -> Date {
        self.base.reference_date()
    }
}
