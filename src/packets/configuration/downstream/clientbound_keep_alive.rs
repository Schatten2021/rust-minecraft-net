use minecraft_net_proc::Packet;
Packet!(ClientBoundKeepAlive, 0x04, {
    id: Long,
});