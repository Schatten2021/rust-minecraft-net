use std::time::{SystemTime, UNIX_EPOCH};
use crate::{Errors, Packet, PacketReader};
use crate::fields::encode_long;
#[derive(Debug)]
pub struct StatusRequest {}
impl Packet for StatusRequest {
    const ID: i32 = 0x00;
    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
    fn from_reader(_reader: &mut PacketReader) -> Result<Self, Errors> {
        Ok(Self {})
    }
}
impl StatusRequest {
    pub fn new() -> Self {
        Self {}
    }
}
#[derive(Debug)]
pub struct PingRequest {
    pub timestamp: i64,
}
impl Packet for PingRequest {
    const ID: i32 = 0x01;
    fn to_bytes(&self) -> Vec<u8> {
        encode_long(self.timestamp)
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self, Errors> {
        Ok(Self {
            timestamp: reader.read_long()
        })
    }
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