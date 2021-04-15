extern crate quantlib;

use quantlib::termstructures::{Compounding, InterestRate};
use quantlib::time::{ActualActual, Frequency};

#[test]
fn equivalent_rate_with_time() {
    let r = 0.01;
    let t = 4.0;
    let dc = ActualActual::default();
    let comp = Compounding::Compounded;
    let freq = Frequency::Annual;

    let int_rate = InterestRate::new(r, dc, comp, freq);

    // Convert rate using semi-annual compounding.
    let new_freq = Frequency::Semiannual;
    let new_int_rate = int_rate.equivalent_rate_with_time(comp, new_freq, t);

    assert_eq!(new_int_rate.rate, 0.0099751242241779);

    // Confirm that DF are sufficiently equivalent.
    let df_int_rate = 1. / int_rate.compound_factor_with_time(t);
    let df_new_int_rate = 1. / new_int_rate.compound_factor_with_time(t);

    assert!((df_int_rate - df_new_int_rate).abs() <= 0.000_000_000_1);
}
