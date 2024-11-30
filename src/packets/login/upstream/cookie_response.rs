use crate::fields::{encode_bool, encode_string, encode_var_int};
use crate::{Errors, Packet, PacketReader};
#[derive(Debug)]
pub struct CookieResponse {
    pub key: String,
    pub has_payload: bool,
    pub payload_len: Option<i32>,
    pub payload: Option<Vec<u8>>,
}
impl Packet for CookieResponse {
    const ID: i32 = 0x04;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_string(self.key.clone());
        res.append(&mut encode_bool(self.has_payload));
        if self.has_payload {
            assert!(self.payload.is_some());
            assert!(self.payload_len.is_some());
            res.append(&mut encode_var_int(self.payload_len.unwrap()));
            res.append(&mut self.payload.clone().unwrap())
        }
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self, Errors> {
        let key = reader.read_string()?;
        let has_payload = reader.read_bool()?;
        if has_payload {
            let payload_len = reader.read_var_int()?;
            Ok(Self {
                key,
                has_payload,
                payload_len: Some(payload_len), 
                payload: Some(reader.read_byte_array(payload_len as usize)),
            })
        } else {
            Ok(Self {key, has_payload, payload_len: None, payload: None})
        }
    }
}