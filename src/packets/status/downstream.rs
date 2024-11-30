use crate::fields::{encode_long, encode_string};
use crate::{Errors, Packet, PacketReader};
#[derive(Debug)]
pub struct StatusResponse {
    pub status: String,
}
impl Packet for StatusResponse {
    const ID: i32 = 0x00;
    fn to_bytes(&self) -> Vec<u8> {
        encode_string(self.status.clone())
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self, Errors> {
        Ok(Self {
            status: reader.read_string()?
        })
    }
}
impl StatusResponse {
    pub fn new(status: String) -> Self {
        Self {status}
    }
}
#[derive(Debug)]
pub struct PingResponse {
    pub timestamp: i64,
}
impl Packet for PingResponse {
    const ID: i32 = 0x01;

    fn to_bytes(&self) -> Vec<u8> {
        encode_long(self.timestamp)
    }

    fn from_reader(reader: &mut PacketReader) -> Result<Self, Errors> {
        Ok(Self { timestamp: reader.read_long() })
    }
}
impl PingResponse {
    pub fn new(timestamp: i64) -> Self {
        Self {timestamp}
    }
}