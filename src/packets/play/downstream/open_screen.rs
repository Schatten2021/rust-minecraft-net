use minecraft_net_proc::Packet;

Packet!(OpenScreen, 0x35, {
    window_id: VarInt,
    window_type: VarInt,
    window_title: String,
});