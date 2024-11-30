use crate::fields::{encode_bool, encode_string, encode_var_int};
use crate::{Packet, PacketReader, Result};

#[derive(Debug)]
pub struct CookieResponse {
    pub key: String,
    pub has_payload: bool,
    pub payload_length: Option<i32>,
    pub payload: Option<Vec<u8>>
}
impl Packet for CookieResponse {
    const ID: i32 = 0x01;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![
            encode_string(self.key.clone()),
            encode_bool(self.has_payload),
        ].iter().flatten().copied().collect();
        if self.has_payload {
            res.append(&mut encode_var_int(self.payload_length.unwrap()));
            res.append(&mut self.payload.clone().unwrap())
        }
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let key = reader.read_string()?;
        let has_payload = reader.read_bool()?;
        if has_payload {
            let payload_length = reader.read_var_int()?;
            Ok(Self {
                key, has_payload,
                payload_length: Some(payload_length),
                payload: Some(reader.read_byte_array(payload_length as usize)),
            })
        } else {
            Ok(Self {key, has_payload, payload: None, payload_length: None})
        }
    }
}
impl CookieResponse {
    pub fn new(key: String, payload: Option<Vec<u8>>) -> Self {
        match payload {
            None => Self {
                key, 
                has_payload: false,
                payload_length: None,
                payload: None,
            },
            Some(p) => Self {
                key,
                has_payload: true,
                payload_length: Some(p.len() as i32),
                payload: Some(p)
            }
        }
    }
}