use super::CashFlow;

pub type Leg = Vec<Box<dyn CashFlow>>;
