use crate::fields::{encode_identifier, encode_var_int};
use crate::{Packet, PacketReader, Result};
#[derive(Debug)]
pub struct FeatureFlags {
    pub total_features: i32,
    pub feature_flags: Vec<String>,
}
impl Packet for FeatureFlags {
    const ID: i32 = 0x0C;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_var_int(self.total_features);
        res.append(&mut self.feature_flags.iter().flat_map(|f| encode_identifier(f.clone())).collect());
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let total_features = reader.read_var_int()?;
        let mut feature_flags = Vec::with_capacity(total_features as usize);
        for _ in 0..total_features {
            feature_flags.push(reader.read_identifier()?);
        }
        Ok(Self {
            total_features,
            feature_flags,
        })
    }
}
impl FeatureFlags {
    pub fn new(flags: Vec<String>) -> Self {
        Self {
            total_features: flags.len() as i32,
            feature_flags: flags
        }
    }
}