use crate::fields::encode_string;
use crate::PacketReader;

pub mod upstream;
pub mod downstream;
#[derive(Debug)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}
impl KnownPack {
    pub fn new(namespace: String, id: String, version: String) -> Self {
        Self {namespace, id, version}
    }
    pub fn from_reader(reader: &mut PacketReader) -> crate::errors::Result<Self> {
        Ok(Self {
            namespace: reader.read_string()?,
            id: reader.read_string()?,
            version: reader.read_string()?,
        })
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_string(self.namespace.clone());
        res.append(&mut encode_string(self.id.clone()));
        res.append(&mut encode_string(self.version.clone()));
        res
    }
}
