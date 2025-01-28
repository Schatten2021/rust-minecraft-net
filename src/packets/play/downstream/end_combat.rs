use minecraft_net_proc::Packet;

Packet!(EndCombat, 0x3C, {
    duration: VarInt,
});