#[derive(PartialEq)]
pub enum Frequency {
    /** null frequency */
    NoFrequency = -1,
    /** only once */
    Once = 0,
    /** once a year */
    Annual = 1,
    /** twice a year */
    Semiannual = 2,
    /** every fourth month */
    EveryFourthMonth = 3,
    /** every third month */
    Quarterly = 4,
    /** every second month */
    Bimonthly = 6,
    /** once a month */
    Monthly = 12,
    /** every fourth week */
    EveryFourthWeek = 13,
    /** every second week */
    Biweekly = 26,
    /** once a week */
    Weekly = 52,
    /** once a day */
    Daily = 365,
    /** some other unknown frequency */
    OtherFrequency = 999,
}

impl Frequency {
    #[inline]
    pub fn to_float(&self) -> f64 {
        match self {
            Frequency::NoFrequency => -1.0,
            Frequency::Once => 0.0,
            Frequency::Annual => 1.0,
            Frequency::Semiannual => 2.0,
            Frequency::EveryFourthMonth => 3.0,
            Frequency::Quarterly => 4.0,
            Frequency::Bimonthly => 6.0,
            Frequency::Monthly => 12.0,
            Frequency::EveryFourthWeek => 13.0,
            Frequency::Biweekly => 26.0,
            Frequency::Weekly => 52.0,
            Frequency::Daily => 365.0,
            Frequency::OtherFrequency => 999.0,
        }
    }
}
