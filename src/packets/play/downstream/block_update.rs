use crate::fields::encode_var_int;
use crate::packets::Position;
use crate::{Packet, PacketReader, Result};

//TODO: add Packet
#[derive(Debug)]
pub struct BlockUpdate {
    pub location: Position,
    pub block_id: i32,
}
impl Packet for BlockUpdate {
    const ID: i32 = 0x09;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.location.to_bytes(),
            encode_var_int(self.block_id),
        ].iter().flatten().cloned().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            location: Position::from_reader(reader)?,
            block_id: reader.read_var_int()?,
        })
    }
}
impl BlockUpdate {
    pub fn new(location: Position, block_id: i32) -> Self {
        Self { location, block_id }
    }
}