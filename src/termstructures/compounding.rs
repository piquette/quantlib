#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Compounding {
    Simple,
    Compounded,
    Continuous,
    SimpleThenCompounded,
    CompoundedThenSimple,
}
