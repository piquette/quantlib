use super::Compounding;
use crate::daycounters::DayCounter;
use crate::time::Date;

pub struct InterestRate {
    pub rate: f64,
    pub day_counter: Box<dyn DayCounter>,
    pub compounding: Compounding,
    pub freq_makes_sense: bool,
    pub freq: i32,
}

impl InterestRate {
    pub fn compound_factor(&self, d1: Date, d2: Date) -> f64 {
        let today = Date::default();
        self.compound_factor_with_ref(d1, d2, today, today)
    }
    pub fn compound_factor_with_ref(
        &self,
        d1: Date,
        d2: Date,
        ref_start: Date,
        ref_end: Date,
    ) -> f64 {
        let t = self.day_counter.year_fraction(d1, d2, ref_start, ref_end);
        self.compound_factor_with_time(t)
    }

    fn compound_factor_with_time(&self, t: f64) -> f64 {
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
        }
    }
}
