use crate::{Packet, PacketReader, Result};
use crate::fields::encode_string;

#[derive(Debug)]
pub struct ServerBoundPluginMessage {
    pub channel: String,
    pub data: Vec<u8>
}

impl Packet for ServerBoundPluginMessage {
    const ID: i32 = 0x02;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_string(self.channel.clone()),
            self.data.clone()
        ].iter().flatten().copied().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            channel: reader.read_string()?,
            data: reader.read_rest(),
        })
    }
}
impl ServerBoundPluginMessage {
    pub fn new(channel: String, data: Vec<u8>) -> Self {
        Self {channel, data}
    }
}