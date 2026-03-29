// Core engine — HIGH CONTENTION hotspot
pub struct Engine { state: u64 }
impl Engine {
    pub fn new() -> Self { Self { state: 0 } }
    pub fn process(&mut self) { self.state += 1; }
}
