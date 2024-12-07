use minecraft_net_proc::Packet;
use crate::packets::configuration::KnownPack;

#[derive(Debug, Packet, Clone)]
#[id = 0x0E]
pub struct ClientBoundKnownPacks {
    #[Var]
    pub count: i32,
    #[len = "count"]
    pub known_packs: Vec<KnownPack>
}
impl ClientBoundKnownPacks {
    pub fn new(known_packs: Vec<KnownPack>) -> Self {
        Self {
            count: known_packs.len() as i32,
            known_packs,
        }
    }
}