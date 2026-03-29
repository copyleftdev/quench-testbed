// CONFLICT TEST — Version A (1774750905)
pub struct Engine { state: u64 }
impl Engine {
    pub fn new() -> Self { Self { state: 42 } }
    pub fn version_a(&self) -> &str { "A" }
}
