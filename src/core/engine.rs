// CONFLICT TEST — Version B (1774746747)
pub struct Engine { state: u64 }
impl Engine {
    pub fn new() -> Self { Self { state: 99 } }
    pub fn version_b(&self) -> &str { "B" }
}
