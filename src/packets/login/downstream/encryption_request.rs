use crate::{Errors, Packet, PacketReader};
use crate::fields::{encode_bool, encode_string, encode_var_int};
#[derive(Debug)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key_length: i32,
    pub public_key: Vec<u8>,
    pub verify_token_length: i32,
    pub verify_token: Vec<u8>,
    pub should_authenticate: bool,
}
impl Packet for EncryptionRequest {
    const ID: i32 = 0x01;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_string(self.server_id.clone());
        res.append(&mut encode_var_int(self.public_key_length));
        res.append(&mut self.public_key.clone());
        res.append(&mut encode_var_int(self.verify_token_length));
        res.append(&mut self.verify_token.clone());
        res.append(&mut encode_bool(self.should_authenticate));
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self, Errors> {
        let id = reader.read_string()?;
        let key_len = reader.read_var_int()?;
        let key = reader.read_byte_array(key_len as usize);
        let token_len = reader.read_var_int()?;
        let token = reader.read_byte_array(token_len as usize);
        Ok(Self {
            server_id: id,
            public_key_length: key_len,
            public_key: key,
            verify_token_length: token_len,
            verify_token: token,
            should_authenticate: reader.read_bool()?,
        })
    }
}
impl EncryptionRequest {
    pub fn new(server_id: String, public_key: Vec<u8>, verify_token: Vec<u8>, should_authenticate: bool) -> Self {
        Self {
            server_id,
            public_key_length: public_key.len() as i32,
            public_key,
            verify_token_length: verify_token.len() as i32,
            verify_token,
            should_authenticate,
        }
    }
}