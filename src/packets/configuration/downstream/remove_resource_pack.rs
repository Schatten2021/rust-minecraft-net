use crate::fields::{encode_bool, encode_uuid};
use crate::{Packet, PacketReader, Result};
#[derive(Debug)]
pub struct RemoveResourcePack {
    pub has_uuid: bool,
    pub uuid: Option<u128>,
}
impl Packet for RemoveResourcePack {
    const ID: i32 = 0x08;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_bool(self.has_uuid);
        if self.has_uuid {
            res.append(&mut encode_uuid(self.uuid.unwrap()));
        }
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let has_uuid = reader.read_bool()?;
        if has_uuid {
            Ok(Self {
                has_uuid,
                uuid: Some(reader.read_uuid()),
            })
        } else {
            Ok(Self {
                has_uuid,
                uuid: None,
            })
        }
    }
}
impl RemoveResourcePack {
    pub fn new(uuid: Option<u128>) -> Self {
        Self {
            has_uuid: uuid.is_some(),
            uuid,
        }
    }
}