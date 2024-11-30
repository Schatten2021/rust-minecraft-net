use crate::fields::encode_var_int;
use crate::{Errors, Packet, PacketReader};
#[derive(Debug)]
pub struct EncryptionResponse {
    pub shared_secret_length: i32,
    pub shared_secret: Vec<u8>,
    pub verify_token_length: i32,
    pub verify_token: Vec<u8>,
}
impl Packet for EncryptionResponse {
    const ID: i32 = 0x00;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_var_int(self.shared_secret_length);
        res.append(&mut self.shared_secret.clone());
        res.append(&mut encode_var_int(self.verify_token_length));
        res.append(&mut self.verify_token.clone());
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self, Errors> {
        let secret_len = reader.read_var_int()?;
        let secret = reader.read_byte_array(secret_len as usize);
        let token_len = reader.read_var_int()?;
        let token = reader.read_byte_array(token_len as usize);
        Ok(Self {
            shared_secret_length: secret_len,
            shared_secret: secret,
            verify_token_length: token_len,
            verify_token: token,
        })
    }
}
impl EncryptionResponse {
    pub fn new(secret: Vec<u8>, token: Vec<u8>) -> Self {
        Self {
            shared_secret_length: secret.len() as i32,
            shared_secret: secret,
            verify_token_length: token.len() as i32,
            verify_token: token,
        }
    }
}