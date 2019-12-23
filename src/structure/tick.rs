use serde::{Deserialize, Serialize};
use speedy::{Readable, Writable};

#[derive(Debug, Clone, Serialize, Deserialize, Readable, Writable)]
pub struct Tick {
    pub timestamp: f64,
    pub price_ips: u64,
    pub volume: u32,
}
