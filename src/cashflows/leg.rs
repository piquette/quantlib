use super::CashFlow;

pub type Leg<CF: CashFlow> = Vec<CF>;
