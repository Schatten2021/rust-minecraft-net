use crate::{Packet, PacketReader, Result};
use crate::fields::encode_var_int;

#[derive(Debug)]
pub struct SetCooldown {
    pub item_id: i32,
    pub cooldown_ticks: i32,
}
impl Packet for SetCooldown {
    const ID: i32 = 0x16;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_var_int(self.item_id),
            encode_var_int(self.cooldown_ticks)
        ].iter().flatten().cloned().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            item_id: reader.read_var_int()?,
            cooldown_ticks: reader.read_var_int()?,
        })
    }
}