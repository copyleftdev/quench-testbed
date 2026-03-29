// Database layer
pub struct Database { connected: bool }
impl Database {
    pub fn connect() -> Self { Self { connected: true } }
}
