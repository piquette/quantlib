use chrono::prelude::*;

#[derive(PartialEq, Copy, Debug, Clone)]
pub struct Date {
    pub d: chrono::prelude::Date<Utc>,
}

impl Date {
    const month_length: [i8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const month_leap_length: [i8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    pub fn is_end_of_month(&self) -> bool {
        //
        let day = self.d.day();
        if day == 0 {
            false
        } else {
            true
        }
    }

    pub fn is_after(&self, other: Date) -> bool {
        true
    }
}
