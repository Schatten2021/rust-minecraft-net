use crate::fields::{encode_identifier, encode_var_int};
use crate::{Packet, PacketReader, Result};
#[derive(Debug)]
pub struct StoreCookie {
    pub key: String,
    pub payload_length: i32,
    pub payload: Vec<u8>,
}
impl Packet for StoreCookie {
    const ID: i32 = 0x0A;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_identifier(self.key.clone());
        res.append(&mut encode_var_int(self.payload_length));
        res.append(&mut self.payload.clone());
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let key = reader.read_identifier()?;
        let payload_length = reader.read_var_int()?;
        let payload = reader.read_byte_array(payload_length as usize);
        Ok(Self { key, payload_length, payload })
    }
}
impl StoreCookie {
    pub fn new(key: String, payload: Vec<u8>) -> Self {
        Self {
            key,
            payload_length: payload.len() as i32,
            payload
        }
    }
}