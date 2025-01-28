use minecraft_net_proc::Packet;

Packet!(SetCooldown, 0x17, {
    item_id: VarInt,
    cooldown_ticks: VarInt,
});