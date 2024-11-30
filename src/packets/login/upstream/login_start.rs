use crate::fields::{encode_string, encode_uuid};
use crate::{Errors, Packet, PacketReader};
#[derive(Debug)]
pub struct LoginStart {
    pub name: String,
    pub uuid: u128,
}
impl Packet for LoginStart {
    const ID: i32 = 0x00;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_string(self.name.clone());
        res.append(&mut encode_uuid(self.uuid));
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self, Errors> {
        Ok(Self {
            name: reader.read_string()?,
            uuid: reader.read_uuid(),
        })
    }
}
impl LoginStart {
    pub fn new(name: String, uuid: u128) -> Self {
        Self {name, uuid}
    }
}