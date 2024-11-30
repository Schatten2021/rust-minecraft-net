use crate::{Errors, Packet, PacketReader};
use crate::fields::encode_var_int;
#[derive(Debug)]
pub struct SetCompression {
    pub threshold: i32,
}
impl Packet for SetCompression {
    const ID: i32 = 0x03;
    fn to_bytes(&self) -> Vec<u8> {
        encode_var_int(self.threshold)
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self, Errors> {
        Ok(Self {threshold: reader.read_var_int()?})
    }
}