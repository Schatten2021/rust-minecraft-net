use minecraft_net_proc::Packet;

Packet!(ServerBoundKeepAlive, 0x04, {
    id: VarLong
});