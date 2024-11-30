use crate::{Result, PacketReader, Packet};
use crate::fields::{encode_uuid, encode_var_int};

#[derive(Debug)]
pub struct ResourcePackResponse {
    pub uuid: u128,
    pub result: i32,
}

impl Packet for ResourcePackResponse {
    const ID: i32 = 0x06;
    fn to_bytes(&self) -> Vec<u8> {
        vec![encode_uuid(self.uuid), encode_var_int(self.result)].iter().flatten().copied().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            uuid: reader.read_uuid(),
            result: reader.read_var_int()?,
        })
    }
}
impl ResourcePackResponse {
    pub fn new(uuid: u128, result: i32) -> Self {
        Self {uuid, result}
    }
}