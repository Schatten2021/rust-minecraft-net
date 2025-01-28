use minecraft_net_proc::Packet;
Packet!(ClientboundKeepAlive, 0x27, {
    keep_alive_id: Long,
});