use crate::fields::encode_string;
use crate::{Packet, PacketReader};
use crate::Result;
#[derive(Debug)]
pub struct ClientboundPluginMessage {
    pub channel: String,
    pub data: Vec<u8>,
}
impl Packet for ClientboundPluginMessage {
    const ID: i32 = 0x01;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_string(self.channel.clone());
        res.append(&mut self.data.clone());
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            channel: reader.read_string()?,
            data: reader.read_rest(),
        })
    }
}
impl ClientboundPluginMessage {
    pub fn new(channel: String, data: Vec<u8>) -> Self {
        Self {channel, data}
    }
}