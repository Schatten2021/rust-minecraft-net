use bytes::Buf;
use crate::{Packet, PacketReader, Result};
use crate::fields::{encode_double, encode_short, encode_var_int};

#[derive(Debug)]
pub struct SpawnExperienceOrb {
    entity_id: i32,
    x: f64,
    y: f64,
    z: f64,
    count: i16,
}
impl Packet for SpawnExperienceOrb {
    const ID: i32 = 0x02;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_var_int(self.entity_id),
            encode_double(self.x),
            encode_double(self.y),
            encode_double(self.z),
            encode_short(self.count),
        ].iter().flatten().copied().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            entity_id: reader.read_var_int()?,
            x: reader.read_double(),
            y: reader.read_double(),
            z: reader.read_double(),
            count: reader.read_short(),
        })
    }
}
impl SpawnExperienceOrb {
    pub fn new(entity_id: i32, x: f64, y: f64, z: f64, count: i16) -> Self {
        Self { entity_id, x, y, z, count }
    }
}