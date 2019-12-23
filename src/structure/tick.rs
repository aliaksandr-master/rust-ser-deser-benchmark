use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tick {
    pub timestamp: f64,
    pub price_ips: u64,
    pub volume: u32,
}
