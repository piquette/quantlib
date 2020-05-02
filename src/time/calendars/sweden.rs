use crate::calendar::Calendar;
use crate::time::Date;
use crate::weekday::Weekday;
use crate::month::Month;

pub struct Sweden;

impl crate::time::traits::Calendar for Sweden {
    fn name(&self) -> String {
        String::from("Sweden")
    }
    fn is_business_day(&self, date: Date) -> bool {
        // Need to impl the actual spec here.
        let wkdy = date.weekday();
        let d = date.day_of_month();
        let dd = date.day_of_year();
        let m = date.month();
        let y = date.year();
        let em = Calendar::easter_monday(y);
        if self.is_weekend(&wkdy) 
         // Good Friday
	                || (dd == em-3)
	                // Easter Monday
	                || (dd == em)
	                // Ascension Thursday
	                || (dd == em+38)
	                // Whit Monday
	                || (dd == em+49)
	                // New Year's Day
	                || (d == 1  && m == Month::January)
	                // Epiphany
	                || (d == 6  && m == Month::January)
	                // May Day
	                || (d == 1  && m == Month::May)
	                // June 6 id National Day but is not a holiday.
	                // It has been debated wheter or not this day should be
	                // declared as a holiday.
	                // As of 2002 the Stockholmborsen is open that day
	                // || (d == 6  && m == June)
	                // Midsummer Eve (Friday between June 18-24)
	                || (wkdy == Weekday::Friday && (d >= 18 && d <= 24) && m == Month::June)
	                // Christmas Eve
	                || (d == 24 && m == Month::December)
	                // Christmas Day
	                || (d == 25 && m == Month::December)
	                // Boxing Day
	                || (d == 26 && m == Month::December)
	                // New Year's Eve
	                || (d == 31 && m == Month::December){
            false
        } else {
            true
        }
    }
    fn is_weekend(&self, weekday: &Weekday) -> bool {
        *weekday == Weekday::Saturday || *weekday == Weekday::Sunday
    }
}
