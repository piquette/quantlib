use super::{BusinessDayConvention, Date, Period, TimeUnit, Weekday};

pub struct Calendar {}

impl Calendar {
    pub fn is_business_day(&self, _date: Date) -> bool {
        true
    }
    pub fn is_holiday(&self, _date: Date) -> bool {
        false
    }
    pub fn is_weekend(&self, _weekday: Weekday) -> bool {
        false
    }
    pub fn is_end_of_month(&self, _date: Date) -> bool {
        false
    }
    pub fn end_of_month(&self, _date: Date) -> Date {
        Date::default()
    }
    pub fn add_holiday(&self, _date: Date) {}

    pub fn remove_holiday(&self, _date: Date) {}

    pub fn adjust(&self, _date: Date) -> Date {
        Date::default()
    }
    pub fn adjust_with_convention(&self, _date: Date, _convention: BusinessDayConvention) -> Date {
        Date::default()
    }
    pub fn advance_with_convention(
        &self,
        date: Date,
        period: Period,
        convention: BusinessDayConvention,
    ) -> Date {
        self.advance_convention_eom(date, period, convention, false)
    }
    pub fn advance_convention_eom(
        &self,
        date: Date,
        period: Period,
        convention: BusinessDayConvention,
        include_end_of_month: bool,
    ) -> Date {
        self.advance(
            date,
            period.length,
            period.units,
            convention,
            include_end_of_month,
        )
    }
    pub fn advance_by_units(&self, date: Date, n: i32, time_unit: TimeUnit) -> Date {
        self.advance(date, n, time_unit, BusinessDayConvention::Following, false)
    }
    pub fn advance_by_period(&self, date: Date, period: Period) -> Date {
        self.advance(
            date,
            period.length,
            period.units,
            BusinessDayConvention::Following,
            false,
        )
    }

    pub fn advance(
        &self,
        _date: Date,
        _n: i32,
        _time_unit: TimeUnit,
        _convention: BusinessDayConvention,
        _include_end_of_month: bool,
    ) -> Date {
        Date::default()
    }

    pub fn business_days_between(&self, from: Date, to: Date) -> i32 {
        self.business_days_between_include(from, to, true, false)
    }
    pub fn business_days_between_include(
        &self,
        _from: Date,
        _to: Date,
        _include_first: bool,
        _include_last: bool,
    ) -> i32 {
        0
    }
}
