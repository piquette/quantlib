use super::month::Month;
use super::weekday::Weekday;
use chrono::prelude::*;
use chrono::Date as ChronDate;
//use chrono::TimeZone as ChronZone;

#[derive(PartialEq, Copy, Debug, Clone, PartialOrd)]
pub struct Date {
    pub d: ChronDate<Utc>,
}

impl Default for Date {
    fn default() -> Date {
        Date { d: Utc::today() }
    }
}
const MONTH_LENGTHS: [usize; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const MONTH_LEAP_LENGTHS: [usize; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const YEAR_IS_LEAP: [bool; 301] = [
    // 1900 is leap in agreement with Excel's bug
    // 1900 is out of valid date range anyway
    // 1900-1909
    true, false, false, false, true, false, false, false, true, false, // 1910-1919
    false, false, true, false, false, false, true, false, false, false, // 1920-1929
    true, false, false, false, true, false, false, false, true, false, // 1930-1939
    false, false, true, false, false, false, true, false, false, false, // 1940-1949
    true, false, false, false, true, false, false, false, true, false, // 1950-1959
    false, false, true, false, false, false, true, false, false, false, // 1960-1969
    true, false, false, false, true, false, false, false, true, false, // 1970-1979
    false, false, true, false, false, false, true, false, false, false, // 1980-1989
    true, false, false, false, true, false, false, false, true, false, // 1990-1999
    false, false, true, false, false, false, true, false, false, false, // 2000-2009
    true, false, false, false, true, false, false, false, true, false, // 2010-2019
    false, false, true, false, false, false, true, false, false, false, // 2020-2029
    true, false, false, false, true, false, false, false, true, false, // 2030-2039
    false, false, true, false, false, false, true, false, false, false, // 2040-2049
    true, false, false, false, true, false, false, false, true, false, // 2050-2059
    false, false, true, false, false, false, true, false, false, false, // 2060-2069
    true, false, false, false, true, false, false, false, true, false, // 2070-2079
    false, false, true, false, false, false, true, false, false, false, // 2080-2089
    true, false, false, false, true, false, false, false, true, false, // 2090-2099
    false, false, true, false, false, false, true, false, false, false, // 2100-2109
    false, false, false, false, true, false, false, false, true, false, // 2110-2119
    false, false, true, false, false, false, true, false, false, false, // 2120-2129
    true, false, false, false, true, false, false, false, true, false, // 2130-2139
    false, false, true, false, false, false, true, false, false, false, // 2140-2149
    true, false, false, false, true, false, false, false, true, false, // 2150-2159
    false, false, true, false, false, false, true, false, false, false, // 2160-2169
    true, false, false, false, true, false, false, false, true, false, // 2170-2179
    false, false, true, false, false, false, true, false, false, false, // 2180-2189
    true, false, false, false, true, false, false, false, true, false, // 2190-2199
    false, false, true, false, false, false, true, false, false, false, // 2200
    false,
];

impl Date {
    pub fn new(day: u32, month: Month, year: i32) -> Date {
        Date {
            d: Utc.ymd(year, month as u32, day),
        }
    }
    pub fn max(&self, d: Date) -> Date {
        if *self > d {
            return *self;
        }
        d
    }
    pub fn sub(&self, date: Date) -> i64 {
        self.d.signed_duration_since(date.d).num_days()
    }
    pub fn day_of_month_zeroed(&self) -> usize {
        (self.d.day() - 1) as usize
    }
    pub fn day_of_month(&self) -> usize {
        self.d.day() as usize
    }
    pub fn month(&self) -> Month {
        Month::from_int(self.d.month()).unwrap()
    }

    pub fn year(&self) -> usize {
        self.d.year() as usize
    }
    pub fn day_of_year(&self) -> usize {
        self.d.ordinal() as usize
    }

    pub fn weekday(&self) -> Weekday {
        Weekday::from_int(self.d.weekday().number_from_sunday()).unwrap()
    }

    pub fn is_end_of_month(date: Date) -> bool {
        //
        let day = date.day_of_month();
        let month = date.month();

        day == Date::month_length(month as usize, Date::is_leap(date.year()))
    }

    pub fn is_leap(year: usize) -> bool {
        YEAR_IS_LEAP[(year - 1900) as usize]
    }

    fn month_length(month: usize, is_leap_year: bool) -> usize {
        if is_leap_year {
            MONTH_LEAP_LENGTHS[(month - 1) as usize]
        } else {
            MONTH_LENGTHS[(month - 1) as usize]
        }
    }
}
