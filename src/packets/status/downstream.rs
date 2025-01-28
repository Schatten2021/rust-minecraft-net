use minecraft_net_proc::Packet;

Packet!(StatusResponse, 0x00, {
    status: String
});
Packet!(PingResponse, 0x01, {
    timestamp: Long
});