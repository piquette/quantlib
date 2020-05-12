use super::traits::Coupon;
use super::{Base, CashFlow, Event};
use crate::termstructures::InterestRate;
use crate::time::{Date, DayCounter};

#[derive(Copy, Clone)]
pub struct FixedRateCoupon<DC: DayCounter> {
    pub base: Base<DC>,
    pub interest_rate: InterestRate<DC>,
}

// impl<DC> Coupon for FixedRateCoupon<DC>
// where
//     DC: DayCounter,
// {
//     fn rate(&self) -> f64 {
//         self.interest_rate.rate
//     }
//     fn accrued_amount(&self, date: Date) -> f64 {
//         if date.d.le(&self.fields.accrual_start_date.d) || date.d.gt(&self.fields.payment_date.d) {
//             0.0
//         } else {
//             let min_date = if date.d.le(&self.fields.accrual_end_date.d) {
//                 date
//             } else {
//                 self.fields.accrual_end_date
//             };
//             self.fields.nominal
//                 * (self.interest_rate.compound_factor_with_ref(
//                     self.fields.accrual_start_date,
//                     min_date,
//                     Some(self.fields.reference_period_start),
//                     Some(self.fields.reference_period_end),
//                 ) - 1.0)
//         }
//     }
//     fn accrual_period(&self) -> f64 {
//         self.fields.day_counter.year_fraction(
//             self.fields.accrual_start_date,
//             self.fields.accrual_end_date,
//             Some(self.fields.reference_period_start),
//             Some(self.fields.reference_period_end),
//         )
//     }
//     fn accrual_days(&self) -> usize {
//         self.fields
//             .day_counter
//             .day_count(self.fields.accrual_start_date, self.fields.accrual_end_date)
//             as usize
//     }
// }

// impl<DC> CashFlow for FixedRateCoupon<DC>
// where
//     DC: DayCounter,
// {
//     fn amount(&self) -> f64 {
//         self.fields.nominal
//             * (self.interest_rate.compound_factor_with_ref(
//                 self.fields.accrual_start_date,
//                 self.fields.accrual_end_date,
//                 Some(self.fields.reference_period_start),
//                 Some(self.fields.reference_period_end),
//             ) - 1.0)
//     }
//     fn try_as_coup(&self) -> Option<&dyn Coupon> {
//         Some(self)
//     }
// }

// impl<DC> Event for FixedRateCoupon<DC>
// where
//     DC: DayCounter,
// {
//     fn date(&self) -> Date {
//         self.fields.payment_date
//     }
//     fn has_occured(&self, date: Date, include_today: bool) -> bool {
//         if include_today {
//             self.fields.payment_date.d.le(&date.d)
//         } else {
//             self.fields.payment_date.d.le(&date.d) || self.fields.payment_date.d.eq(&date.d)
//         }
//     }
// }
