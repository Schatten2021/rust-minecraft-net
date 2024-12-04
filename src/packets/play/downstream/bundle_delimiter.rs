use crate::{Packet, PacketReader, Result};
#[derive(Debug)]
pub struct BundleDelimiter;

impl Packet for BundleDelimiter {
    const ID: i32 = 0x00;
    fn to_bytes(&self) -> Vec<u8> {vec![]}
    fn from_reader(_reader: &mut PacketReader) -> Result<Self> { Ok(Self{}) }
}
impl BundleDelimiter {
    pub fn new() -> Self {Self {}}
}