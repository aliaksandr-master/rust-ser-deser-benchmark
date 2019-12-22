use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tick {
    timestamp: f64,
    price_ips: u64,
    volume: u32,
}
