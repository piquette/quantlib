pub trait PricingEngine {
    fn get_results(&self) -> Box<dyn Results>;
    fn get_arguments(&self) -> Box<dyn Arguments>;
    fn reset(&self);
    fn update(&self);
    fn calculate(&self);
}

pub trait Results {
    fn reset(&self);
}

pub trait Arguments {
    fn validate(&self);
}
