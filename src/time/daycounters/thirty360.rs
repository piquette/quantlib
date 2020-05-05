use crate::time::traits::*;
use crate::time::Date;
use std::cmp;

pub enum Convention360 {
    USA,
    BondBasis,
    European,
    EurobondBasis,
    Italian,
}

//
// 30/360 day count convention.
// The 30/360 day count can be calculated according to US, European, or Italian
// conventions.
//
// US (NASD) convention: if the starting date is the 31st of a month, it becomes
// equal to the 30th of the same month. If the ending date is the 31st of a
// month and the starting date is earlier than the 30th of a month, the ending
// date becomes equal to the 1st of the next month, otherwise the ending date
// becomes equal to the 30th of the same month. Also known as "30/360",
// "360/360", or "Bond Basis"
//
// European convention: starting dates or ending dates that occur on the 31st of
// a month become equal to the 30th of the same month. Also known as "30E/360",
// or "Eurobond Basis"
//
// Italian convention: starting dates or ending dates that occur on February and
// are grater than 27 become equal to 30 for computational sake.
//
// http://en.wikipedia.org/wiki/Day_count_convention
//
pub struct Thirty360 {
    pub convention: Convention360,
}
impl Default for Thirty360 {
    fn default() -> Thirty360 {
        Thirty360 {
            convention: Convention360::BondBasis,
        }
    }
}

//
//
//
impl DayCounter for Thirty360 {
    //
    //
    //
    fn day_count(&self, date_start: Date, date_end: Date) -> i64 {
        let mut dm1 = date_start.day_of_month();
        let mut dm2 = date_end.day_of_month();
        let m1 = date_start.month() as usize;
        let mut m2 = date_end.month() as usize;
        let y1 = date_start.year();
        let y2 = date_end.year();

        match self.convention {
            // US and regular bonds.
            // =====================
            Convention360::USA | Convention360::BondBasis => {
                if dm2 == 31 && dm1 < 30 {
                    dm2 = 1;
                    m2 = m2 + 1; // clearly this could go wrong if it was december?
                }

                (360 * (y2 - y1)
                    + 30 * (m2 as usize - m1 as usize - 1)
                    + cmp::max(0, 30 - dm1) as usize
                    + cmp::min(30, dm2) as usize) as i64
            }
            // European and euro bonds.
            // =====================
            Convention360::European | Convention360::EurobondBasis => {
                (360 * (y2 - y1)
                    + 30 * (m2 as usize - m1 as usize - 1)
                    + cmp::max(0, 30 - dm1) as usize
                    + cmp::min(30, dm2) as usize) as i64
            }
            // Italian bonds.
            // =====================
            Convention360::Italian => {
                // adjust february.
                if m1 == 2 && dm1 > 27 {
                    dm1 = 30;
                }
                if m2 == 2 && dm2 > 27 {
                    dm2 = 30;
                }

                (360 * (y2 - y1)
                    + 30 * (m2 as usize - m1 as usize - 1)
                    + cmp::max(0, 30 - dm1) as usize
                    + cmp::min(30, dm2) as usize) as i64
            }
        }
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
        self.day_count(date_start, date_end) as f64 / 360.0
    }
}
