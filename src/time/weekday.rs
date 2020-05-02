#[derive(PartialEq)]
pub enum Weekday {
    Sunday = 1,
    Monday = 2,
    Tuesday = 3,
    Wednesday = 4,
    Thursday = 5,
    Friday = 6,
    Saturday = 7,
}

impl Weekday {
    #[inline]
    pub fn from_int(n: u32) -> Option<Weekday> {
        match n {
            1 => Some(Weekday::Sunday),
            2 => Some(Weekday::Monday),
            3 => Some(Weekday::Tuesday),
            4 => Some(Weekday::Wednesday),
            5 => Some(Weekday::Thursday),
            6 => Some(Weekday::Friday),
            7 => Some(Weekday::Saturday),
            _ => None,
        }
    }
}
