use std::fmt::Pointer;
use crate::{Packet, PacketReader, Result};
use crate::fields::{encode_float, encode_ubyte};

#[derive(Debug)]
pub struct GameEvent {
    event: u8,
    value: f32,
}
impl Packet for GameEvent {
    const ID: i32 = 0x22;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_ubyte(self.event),
            encode_float(self.value),
        ].iter().flatten().cloned().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            event: reader.read_ubyte(),
            value: reader.read_float(),
        })
    }
}
impl GameEvent {
    pub fn new(event: u8, value: f32) -> Self {
        Self { event, value }
    }
}