use minecraft_net_proc::Packet;

Packet!(SetCompression, 0x03, {
    threshold: VarInt,
});