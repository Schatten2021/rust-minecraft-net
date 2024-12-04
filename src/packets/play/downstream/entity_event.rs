use crate::{Packet, PacketReader, Result};
use crate::fields::{encode_byte, encode_int};

#[derive(Debug)]
pub struct EntityEvent {
    entity_id: i32,
    entity_status: i8,
}
impl Packet for EntityEvent {
    const ID: i32 = 0x1F;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_int(self.entity_id),
            encode_byte(self.entity_status),
        ].iter().flatten().cloned().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            entity_id: reader.read_int(),
            entity_status: reader.read_byte(),
        })
    }
}
impl EntityEvent {
    pub fn new(entity_id: i32, entity_status: i8) -> Self {
        Self {entity_id, entity_status}
    }
}