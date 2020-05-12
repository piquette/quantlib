#[derive(Default, Clone)]
pub struct LazyObject {
    pub calculated: bool,
    pub frozen: bool,
    pub always_forward: bool,
}

impl LazyObject {
    pub fn update(&mut self) {}
    pub fn recalculate(&mut self) {}
    pub fn calculate(&mut self) {}
    pub fn freeze(&mut self) {}
    pub fn unfreeze(&mut self) {}
}
