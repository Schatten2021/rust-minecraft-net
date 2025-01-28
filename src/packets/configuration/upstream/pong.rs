use minecraft_net_proc::Packet;

Packet!(Pong, 0x05, {
    id: VarInt
});