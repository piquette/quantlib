use super::day_count;
use crate::time::traits::*;
use crate::time::Date;

pub enum ConventionActual {
    ISMA,
    Bond,
    ISDA,
    Historical,
    Actual365,
    AFB,
    Euro,
}

//
// ActualActual day counters.
//
// http://en.wikipedia.org/wiki/Day_count_convention
//
pub struct ActualActual {
    pub convention: ConventionActual,
}
impl Default for ActualActual {
    fn default() -> ActualActual {
        ActualActual {
            convention: ConventionActual::ISDA,
        }
    }
}

//
//
//
impl DayCounter for ActualActual {
    //
    //
    //
    fn day_count(&self, date_start: Date, date_end: Date) -> i64 {
        day_count(date_start, date_end)
    }

    //
    //
    //
    //
    fn year_fraction(
        &self,
        date_start: Date,
        date_end: Date,
        _ref_period_start: Option<Date>,
        _ref_period_end: Option<Date>,
    ) -> f64 {
        let mut dm1 = date_start.day_of_month();
        let mut dm2 = date_end.day_of_month();
        let m1 = date_start.month();
        let mut m2 = date_end.month();
        let y1 = date_start.year();
        let y2 = date_end.year();

        match self.convention {
            ConventionActual::ISMA | ConventionActual::Bond => 0.0,
            ConventionActual::ISDA | ConventionActual::Actual365 | ConventionActual::Historical => {
                0.0
            }
            ConventionActual::AFB | ConventionActual::Euro => 0.0,
        }
    }
}
