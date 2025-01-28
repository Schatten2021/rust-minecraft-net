use minecraft_net_proc::Packet;
Packet!(CombatDeath, 0x3E, {
    player_id: VarInt,
    message: String,
});