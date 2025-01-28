use minecraft_net_proc::Packet;
Packet!(Disconect, 0x1D, {
    reason: String,
});