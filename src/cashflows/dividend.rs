use super::CashFlow;

pub trait Dividend: CashFlow {
    fn pro_rata_amount(self, underlying: f64) -> f64;
}
