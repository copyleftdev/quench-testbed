// Core engine — modified by QUENCH e2e test at 2026-03-29 01:07:19.491832003 UTC
pub struct Engine { state: u64 }
impl Engine {
    pub fn new() -> Self { Self { state: 0 } }
    pub fn process(&mut self) { self.state += 1; }
    pub fn reset(&mut self) { self.state = 0; } // NEW
}
