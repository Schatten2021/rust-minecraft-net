use minecraft_net_proc::Packet;
use crate::fields::types::TextComponent;

Packet!(CombatDeath, 0x3E, {
    player_id: VarInt,
    message: TextComponent,
});