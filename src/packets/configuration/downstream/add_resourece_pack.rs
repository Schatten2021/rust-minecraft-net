use crate::fields::{encode_bool, encode_string, encode_uuid};
use crate::{Packet, PacketReader, Result};
#[derive(Debug)]

pub struct AddResourcePack {
    pub uuid: u128,
    pub url: String,
    pub hash: String,
    pub forced: bool,
    pub has_prompt_message: bool,
    pub prompt_message: Option<String>,
}
impl Packet for AddResourcePack {
    const ID: i32 = 0x09;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_uuid(self.uuid);
        res.append(&mut encode_string(self.url.clone()));
        res.append(&mut encode_string(self.hash.clone()));
        res.append(&mut encode_bool(self.forced));
        res.append(&mut encode_bool(self.has_prompt_message));
        if self.has_prompt_message {
            res.append(&mut encode_string(self.prompt_message.clone().unwrap()));
        }
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let uuid = reader.read_uuid();
        let url = reader.read_string()?;
        let hash = reader.read_string()?;
        let forced = reader.read_bool()?;
        let has_prompt_message = reader.read_bool()?;
        if has_prompt_message {
            Ok(Self {
                uuid, url, hash, forced, has_prompt_message,
                prompt_message: Some(reader.read_string()?)
            })
        } else {
            Ok(Self {
                uuid, url, hash, forced, has_prompt_message,
                prompt_message: None,
            })
        }
    }
}
impl AddResourcePack {
    pub fn new(uuid: u128, url: String, hash: String, forced: bool, prompt_message: Option<String>) -> Self {
        Self {
            uuid, url, hash, forced,
            has_prompt_message: prompt_message.is_some(),
            prompt_message,
        }
    }
}