use minecraft_net_proc::Packet;
Packet!(Transfer, 0x0B, {
    host: String,
    port: VarInt,
});