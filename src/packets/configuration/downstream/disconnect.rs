use minecraft_net_proc::Packet;
Packet!(Disconnect, 0x02, {
    reason: String,
});