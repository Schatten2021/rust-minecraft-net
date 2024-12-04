use crate::{Packet, PacketReader, Result};
use crate::fields::{encode_float, encode_var_int};

#[derive(Debug)]
pub struct HurtAnimation {
    pub entity_id: i32,
    pub yaw: f32
}
impl Packet for HurtAnimation {
    const ID: i32 = 0x24;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_var_int(self.entity_id),
            encode_float(self.yaw),
        ].iter().flatten().cloned().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            entity_id: reader.read_var_int()?,
            yaw: reader.read_float(),
        })
    }
}
impl HurtAnimation {
    pub fn new(entity_id: i32, yaw: f32) -> Self {
        Self {entity_id, yaw}
    }
}