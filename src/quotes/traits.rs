pub trait Quote {
    fn value(&self) -> f64;
    fn is_valid(&self) -> bool;
}
