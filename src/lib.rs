//! Library to used for quantitative finance.
#![allow(dead_code)]
#![deny(//missing_docs,
        unsafe_code,
        unused_import_braces,
        unused_qualifications)]

#[macro_use]
pub mod cashflows;
pub mod currencies;
pub mod definitions;
pub mod instruments;
pub mod patterns;
pub mod pricingengines;
pub mod quotes;
pub mod termstructures;
pub mod time;

pub use self::time::*;
