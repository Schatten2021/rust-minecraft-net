use crate::{Packet, PacketReader, Result};
use crate::fields::{encode_short, encode_ubyte};

#[derive(Debug)]
pub struct SetContainerProperty {
    pub window_id: u8,
    pub property: i16,
    pub value: i16,
}

impl Packet for SetContainerProperty {
    const ID: i32 = 0x14;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_ubyte(self.window_id),
            encode_short(self.property),
            encode_short(self.value)
        ].iter().flatten().cloned().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            window_id: reader.read_ubyte(),
            property: reader.read_short(),
            value: reader.read_short(),
        })
    }
}
impl SetContainerProperty {
    pub fn new(window_id: u8, property: i16, value: i16) -> Self {
        Self {window_id, property, value}
    }
}