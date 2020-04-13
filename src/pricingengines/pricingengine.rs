pub trait PricingEngine {
    fn get_results();
    fn get_arguments();
    fn reset();
    fn update();
    fn calculate();
}
