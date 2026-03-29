// Agent A change
pub struct Database { connected: bool }
impl Database {
    pub fn connect() -> Self { Self { connected: true } }
    pub fn query(&self) -> &str { "result-1774746736" }
}
