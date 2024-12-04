use crate::{Packet, PacketReader, Result};
use crate::fields::{encode_ubyte, encode_var_int};

#[derive(Debug)]
pub struct EntityAnimation {
    entity_id: i32,
    animation: u8,
}
impl Packet for EntityAnimation {
    const ID: i32 = 0x03;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_var_int(self.entity_id),
            encode_ubyte(self.animation)
        ].iter().flatten().cloned().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            entity_id: reader.read_var_int()?,
            animation: reader.read_ubyte(),
        })
    }
}
impl EntityAnimation {
    pub fn new(entity_id: i32, animation: u8) -> Self {
        Self {entity_id, animation}
    }
}