use crate::fields::encode_var_int;
use crate::packets::configuration::KnownPack;
use crate::{Packet, PacketReader, Result};

#[derive(Debug)]
pub struct ServerBoundKnownPacks {
    pub count: i32,
    pub known_packs: Vec<KnownPack>,
}
impl Packet for ServerBoundKnownPacks {
    const ID: i32 = 0x07;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_var_int(self.count),
            self.known_packs.iter().flat_map(|p| p.to_bytes()).collect()
        ].iter().flatten().copied().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let count = reader.read_var_int()?;
        let mut known_packs = Vec::with_capacity(count as usize);
        for _ in 0..count {
            known_packs.push(KnownPack::from_reader(reader)?)
        }
        Ok(Self {count, known_packs})
    }
}
impl ServerBoundKnownPacks {
    pub fn new(known_packs: Vec<KnownPack>) -> Self {
        Self {
            count: known_packs.len() as i32,
            known_packs,
        }
    }
}