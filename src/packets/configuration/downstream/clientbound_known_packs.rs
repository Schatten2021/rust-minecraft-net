use crate::fields::encode_var_int;
use crate::packets::configuration::KnownPack;
use crate::{Packet, PacketReader, Result};

#[derive(Debug)]
pub struct ClientBoundKnownPacks {
    pub count: i32,
    pub known_packs: Vec<KnownPack>
}
impl Packet for ClientBoundKnownPacks {
    const ID: i32 = 0x0E;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_var_int(self.count);
        res.append(&mut self.known_packs.iter().flat_map(|p| p.to_bytes()).collect());
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let count = reader.read_var_int()?;
        let mut known_packs = Vec::with_capacity(count as usize);
        for _ in 0..count {
            known_packs.push(KnownPack::from_reader(reader)?);
        }
        Ok(Self { count, known_packs })
    }
}
impl ClientBoundKnownPacks {
    pub fn new(known_packs: Vec<KnownPack>) -> Self {
        Self {
            count: known_packs.len() as i32,
            known_packs,
        }
    }
}