// Configuration — shared module
pub struct Config {
    pub max_workers: usize,
    pub timeout_ms: u64,
}
impl Default for Config {
    fn default() -> Self { Self { max_workers: 4, timeout_ms: 5000 } }
}
