use crate::packets::configuration::KnownPack;
use minecraft_net_proc::Packet;

#[derive(Debug, Packet, Clone)]
#[id = 0x07]
pub struct ServerBoundKnownPacks {
    #[Var]
    pub count: i32,
    #[len = "count"]
    pub known_packs: Vec<KnownPack>,
}
impl ServerBoundKnownPacks {
    pub fn new(known_packs: Vec<KnownPack>) -> Self {
        Self {
            count: known_packs.len() as i32,
            known_packs,
        }
    }
}