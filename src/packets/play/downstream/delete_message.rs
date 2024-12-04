use crate::{Packet, PacketReader, Result};
use crate::fields::encode_var_int;

#[derive(Debug)]
pub struct DeleteMessage {
    message_id: i32,
    signature: Option<Vec<u8>>
}
impl Packet for DeleteMessage {
    const ID: i32 = 0x1C;
    fn to_bytes(&self) -> Vec<u8> {
        if self.message_id == 0 {
            self.signature.clone().unwrap()
        } else {
            encode_var_int(self.message_id)
        }
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let message_id = reader.read_var_int()?;
        if message_id == 0 {
            Ok(Self {
                message_id,
                signature: Some(reader.read_byte_array(256))
            })
        } else {
            Ok(Self {
                message_id,
                signature: None
            })
        }
    }
}
impl DeleteMessage {
    pub fn new(message_id: i32, signature: Option<Vec<u8>>) -> Self {
        Self {message_id, signature}
    }
}