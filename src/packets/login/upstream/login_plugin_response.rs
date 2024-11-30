use crate::fields::{encode_bool, encode_var_int};
use crate::{Errors, Packet, PacketReader};
#[derive(Debug)]
pub struct LoginPluginResponse {
    pub message_id: i32,
    pub success: bool,
    pub data: Vec<u8>,
}
impl Packet for LoginPluginResponse {
    const ID: i32 = 0x02;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_var_int(self.message_id);
        res.append(&mut encode_bool(self.success));
        res.append(&mut self.data.clone());
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self, Errors> {
        Ok(Self {
            message_id: reader.read_var_int()?,
            success: reader.read_bool()?,
            data: reader.read_rest(),
        })
    }
}
impl LoginPluginResponse {
    pub fn new(message_id: i32, success: bool, data: Vec<u8>) -> Self {
        Self {message_id, success, data}
    }
}