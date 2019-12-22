use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tick {
    timestamp: f64,
    price_ips: u64,
    volume: u32,
}

impl Tick {
    pub fn timestamp(&self) -> f64 {
        self.timestamp
    }

    pub fn price_ips(&self) -> u64 {
        self.price_ips
    }

    pub fn volume(&self) -> u32 {
        self.volume
    }
}
