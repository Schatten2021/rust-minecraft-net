use crate::{Packet, PacketReader, Result};
use crate::fields::{encode_int, encode_ubyte, encode_var_int};

#[derive(Debug)]
pub struct OpenHorseScreen {
    window_id: u8,
    slot_count: i32,
    entity_id: i32,
}

impl Packet for OpenHorseScreen {
    const ID: i32 = 0x23;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_ubyte(self.window_id),
            encode_var_int(self.slot_count),
            encode_int(self.entity_id)
        ].iter().flatten().cloned().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            window_id: reader.read_ubyte(),
            slot_count: reader.read_var_int()?,
            entity_id: reader.read_int(),
        })
    }
}
impl OpenHorseScreen {
    pub fn new(window_id: u8, slot_count: i32, entity_id: i32) -> Self {
        Self {window_id, slot_count, entity_id}
    }
}