use minecraft_net_proc::Packet;
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Debug, Packet)]
#[id = 0x00]
pub struct StatusRequest {}
impl StatusRequest {
    pub fn new() -> Self {
        Self {}
    }
}
#[derive(Debug, Packet)]
#[id = 0x01]
pub struct PingRequest {
    #[Const]
    pub timestamp: i64,
}
impl PingRequest {
    pub fn new(timestamp: i64) -> Self {
        Self {timestamp}
    }
    pub fn now() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
        Self {timestamp: now as i64}
    }
}