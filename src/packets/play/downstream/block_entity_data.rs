use crate::{Errors, Packet, PacketReader, Result};
use crate::fields::encode_var_int;
use crate::packets::Position;

#[derive(Debug)]
pub struct BlockEntityData {
    location: Position,
    r#type: i32,
    data: crab_nbt::Nbt,
}

impl Packet for BlockEntityData {
    const ID: i32 = 0x07;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.location.to_bytes(),
            encode_var_int(self.r#type),
            self.data.serialize_content().into(),
        ].iter().flatten().cloned().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            location: Position::from_reader(reader)?,
            r#type: reader.read_var_int()?,
            data: crab_nbt::Nbt::read(reader).map_err(|e| Errors::NbtError(e))?
        })
    }
}
impl BlockEntityData {
    pub fn new(location: Position, r#type: i32, data: crab_nbt::Nbt) -> Self {
        Self {location, r#type, data}
    }
}