use minecraft_net_proc::Packet;

Packet!(PingResponse, 0x38, {
    payload: Long,
});