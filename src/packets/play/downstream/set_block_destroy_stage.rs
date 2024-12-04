use crate::{Packet, PacketReader, Result};
use crate::fields::{encode_byte, encode_var_int};
use crate::packets::Position;

#[derive(Debug)]
pub struct SetBlockDestroyStage {
    entity_id: i32,
    location: Position,
    destroy_stage: i8
}
impl Packet for SetBlockDestroyStage {
    const ID: i32 = 0x06;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_var_int(self.entity_id),
            self.location.to_bytes(),
            encode_byte(self.destroy_stage),
        ].iter().flatten().cloned().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            entity_id: reader.read_var_int()?,
            location: Position::from_reader(reader)?,
            destroy_stage: reader.read_byte()
        })
    }
}
impl SetBlockDestroyStage {
    pub fn new(entity_id: i32, location: Position, destroy_stage: i8) -> Self {
        Self {entity_id, location, destroy_stage}
    }
}