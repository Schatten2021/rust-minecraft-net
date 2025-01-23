use minecraft_net_proc::{Field, Packet};
use crate::fields::types::{PrefixedArray, UUID};

#[derive(Clone, Debug, Field)]
pub struct Player {
    pub uuid: UUID,
    // pub player_actions: Vec<> TODO: figure out
}

#[derive(Debug, Packet)]
#[id = 0x40]
pub struct PlayerInfoUpdate {
    // actions: EnumSet TODO: figure out
    players: PrefixedArray<Player>
}