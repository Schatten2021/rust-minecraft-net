use minecraft_net_proc::Packet;
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Debug, Packet)]
#[id = 0x00]
pub struct StatusResponse {
    pub status: String,
}
impl StatusResponse {
    pub fn new(status: String) -> Self {
        Self {status}
    }
}
#[derive(Debug, Packet)]
#[id = 0x01]
pub struct PingResponse {
    #[Const]
    pub timestamp: i64,
}
impl PingResponse {
    pub fn new(timestamp: i64) -> Self {
        Self {timestamp}
    }
    pub fn now() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
        Self {timestamp: now as i64}
    }
}