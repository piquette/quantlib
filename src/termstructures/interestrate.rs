use super::Compounding;
use crate::definitions::{Rate, Time};
use crate::time::{Actual365Fixed, Date, DayCounter, Frequency};

#[derive(Copy, Clone)]
pub struct InterestRate<DC: DayCounter> {
    pub rate: Rate,
    pub day_counter: DC,
    pub compounding: Compounding,
    pub freq_makes_sense: bool,
    pub freq: f64,
}
impl<DC> InterestRate<DC>
where
    DC: DayCounter,
{
    pub fn new(r: Rate, day_counter: DC, comp: Compounding, freq: Frequency) -> InterestRate<DC> {
        let mut makes_sense = false;
        if comp == Compounding::Compounded
            || comp == Compounding::SimpleThenCompounded
            || comp == Compounding::CompoundedThenSimple
        {
            makes_sense = true;
            // frequency not allowed for this interest rate.
            assert!(freq != Frequency::Once && freq != Frequency::NoFrequency);
        }
        InterestRate {
            rate: r,
            day_counter: day_counter,
            compounding: comp,
            freq_makes_sense: makes_sense,
            freq: freq.to_float(),
        }
    }

    pub fn implied_rate_with_time(
        compound: f64,
        day_counter: DC,
        comp: Compounding,
        freq: Frequency,
        t: Time,
    ) -> InterestRate<DC> {
        // cant be less than zero.
        assert!(compound > 0.0);
        assert!(t > 0.0);

        let r: Rate;
        if compound == 1.0 {
            r = 0.0;
        } else {
            match comp {
                Compounding::Simple => r = (compound - 1.0) / t,
                Compounding::Compounded => {
                    r = (compound.powf(1.0 / (freq.to_float() * t)) - 1.0) * freq.to_float()
                }
                Compounding::Continuous => r = compound.ln() / t,
                Compounding::SimpleThenCompounded => {
                    if t <= 1.0 / freq.to_float() {
                        r = (compound - 1.0) / t;
                    } else {
                        r = (compound.powf(1.0 / (freq.to_float() * t)) - 1.0) * freq.to_float()
                    }
                }
                Compounding::CompoundedThenSimple => {
                    if t > 1.0 / freq.to_float() {
                        r = (compound - 1.0) / t;
                    } else {
                        r = (compound.powf(1.0 / (freq.to_float() * t)) - 1.0) * freq.to_float()
                    }
                }
            }
        }
        return Self::new(r, day_counter, comp, freq);
    }

    pub fn implied_rate(
        compound: f64,
        day_counter: DC,
        comp: Compounding,
        freq: Frequency,
        date_start: Date,
        date_end: Date,
        ref_period_start: Option<Date>,
        ref_period_end: Option<Date>,
    ) -> InterestRate<DC> {
        assert!(date_end >= date_start);
        let t = day_counter.year_fraction(date_start, date_end, ref_period_start, ref_period_end);
        Self::implied_rate_with_time(compound, day_counter, comp, freq, t)
    }

    pub fn compound_factor(&self, d1: Date, d2: Date) -> f64 {
        self.compound_factor_with_ref(d1, d2, None, None)
    }

    pub fn compound_factor_with_ref(
        &self,
        d1: Date,
        d2: Date,
        ref_start: Option<Date>,
        ref_end: Option<Date>,
    ) -> f64 {
        let t = self.day_counter.year_fraction(d1, d2, ref_start, ref_end);
        self.compound_factor_with_time(t)
    }

    pub fn compound_factor_with_time(&self, t: Time) -> f64 {
        assert!(t >= 0.0);
        match self.compounding {
            Compounding::Simple => {
                // 1+r*t
                1.0 + self.rate * t
            }
            Compounding::Compounded => {
                // (1+r/f)^(f*t)
                (1.0 + self.rate / self.freq as f64).powf(self.freq as f64 * t)
            }
            Compounding::Continuous => {
                // e^(r*t)
                let one = 1.0_f64;
                let e = one.exp();
                e.powf(self.rate * t)
            }
            Compounding::SimpleThenCompounded => {
                if t < (1.0 / self.freq as f64) {
                    // 1+r*t
                    1.0 + self.rate * t
                } else {
                    // (1+(r/f))^(f*t)
                    (1.0 + self.rate / self.freq as f64).powf(self.freq as f64 * t)
                }
            }
            Compounding::CompoundedThenSimple => {
                if t > (1.0 / self.freq as f64) {
                    // 1+r*t
                    1.0 + self.rate * t
                } else {
                    // (1+(r/f))^(f*t)
                    (1.0 + self.rate / self.freq as f64).powf(self.freq as f64 * t)
                }
            }
        }
    }

    pub fn equivalent_rate_with_time(
        &self,
        comp: Compounding,
        freq: Frequency,
        t: Time,
    ) -> InterestRate<DC> {
        Self::implied_rate_with_time(
            self.compound_factor_with_time(t),
            self.day_counter,
            comp,
            freq,
            t,
        )
    }
}
