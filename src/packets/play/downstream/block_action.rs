use std::panic::Location;
use crate::{Errors, Packet, PacketReader, Result};
use crate::fields::{encode_ubyte, encode_var_int};
use crate::packets::Position;

#[derive(Debug)]
pub struct BlockAction {
    pub location: Position,
    pub action_id: u8,
    pub action_parameter: u8,
    pub block_type: i32,
}

impl Packet for BlockAction {
    const ID: i32 = 0x08;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.location.to_bytes(),
            encode_ubyte(self.action_id),
            encode_ubyte(self.action_parameter),
            encode_var_int(self.block_type),
        ].iter().flatten().cloned().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            location: Position::from_reader(reader)?,
            action_id: reader.read_ubyte(),
            action_parameter: reader.read_ubyte(),
            block_type: reader.read_var_int()?,
        })
    }
}
impl BlockAction {
    pub fn new(location: Position, action_id: u8, action_parameter: u8, block_type: i32) -> Self {
        Self {location, action_id, action_parameter, block_type}
    }
}