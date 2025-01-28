use minecraft_net_proc::Packet;
Packet!(OpenBook, 0x34, {
    hand: VarInt,
});