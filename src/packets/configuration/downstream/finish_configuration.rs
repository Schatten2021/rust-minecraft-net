use crate::Result;
use crate::{Packet, PacketReader};
#[derive(Debug)]
pub struct FinishConfiguration {}
impl Packet for FinishConfiguration {
    const ID: i32 = 0x03;
    fn to_bytes(&self) -> Vec<u8> {vec![]}
    fn from_reader(_reader: &mut PacketReader) -> Result<Self> { Ok(Self {  }) }
}
impl FinishConfiguration {
    pub fn new() -> Self {Self{} }
}