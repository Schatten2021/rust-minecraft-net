use crate::fields::{encode_bool, encode_int};
use crate::packets::Position;
use crate::{join, Packet, PacketReader, Result};

//TODO: add Packet
#[derive(Debug)]
pub struct WorldEvent {
    pub event: i32,
    pub location: Position,
    pub data: i32,
    pub disable_relative_volume: bool,
}
impl Packet for WorldEvent {
    const ID: i32 = 0x28;
    fn to_bytes(&self) -> Vec<u8> {
        join!(
            encode_int(self.event),
            self.location.to_bytes(),
            encode_int(self.data),
            encode_bool(self.disable_relative_volume)
        )
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            event: reader.read_int(),
            location: Position::from_reader(reader)?,
            data: reader.read_int(),
            disable_relative_volume: reader.read_bool()?,
        })
    }
}
impl WorldEvent {
    pub fn new(event: i32, location: Position, data: i32, disable_relative_volume: bool) -> Self {
        Self {event, location, data, disable_relative_volume}
    }
}