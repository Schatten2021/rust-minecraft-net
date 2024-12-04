use crate::{Packet, PacketReader, Result};
use crate::fields::{encode_bool, encode_string, encode_var_int};

#[derive(Debug)]
pub struct DisguisedChatMessage {
    pub message: String,
    pub chat_type: i32,
    pub sender_name: String,
    pub has_target: bool,
    pub target: String, //TODO: maybe Option<String>?
}
impl Packet for DisguisedChatMessage {
    const ID: i32 = 0x1E;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_string(self.message.clone()),
            encode_var_int(self.chat_type),
            encode_string(self.sender_name.clone()),
            encode_bool(self.has_target),
            encode_string(self.target.clone()),
        ].iter().flatten().cloned().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            message: reader.read_string()?,
            chat_type: reader.read_var_int()?,
            sender_name: reader.read_string()?,
            has_target: reader.read_bool()?,
            target: reader.read_string()?,
        })
    }
}
impl DisguisedChatMessage {
    pub fn new(message: String, chat_type: i32, sender_name: String, has_target: bool, target: String) -> Self {
        Self {message, chat_type, sender_name, has_target, target}
    }
}