use minecraft_net_proc::Packet;

Packet!(Ping, 0x37, {
    id: Int,
});