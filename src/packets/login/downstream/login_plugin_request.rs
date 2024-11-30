use crate::fields::{encode_identifier, encode_var_int};
use crate::{Errors, Packet, PacketReader};
#[derive(Debug)]
pub struct LoginPluginRequest {
    pub message_id: i32,
    pub channel: String,
    pub data: Vec<u8>,
}
impl Packet for LoginPluginRequest {
    const ID: i32 = 0x04;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_var_int(self.message_id);
        res.append(&mut encode_identifier(self.channel.clone()));
        res.append(&mut self.data.clone());
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self, Errors> {
        let message_id = reader.read_var_int()?;
        let channel = reader.read_string()?;
        let data = reader.read_rest();
        Ok(Self { message_id, channel, data })
    }
}
impl LoginPluginRequest {
    pub fn new(id: i32, channel: String, data: Vec<u8>) -> Self {
        Self {message_id: id, channel, data}
    }
}