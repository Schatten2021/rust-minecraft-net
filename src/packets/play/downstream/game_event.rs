use minecraft_net_proc::Packet;

Packet!(GameEvent, 0x23, {
    event: UByte,
    value: Float,
});