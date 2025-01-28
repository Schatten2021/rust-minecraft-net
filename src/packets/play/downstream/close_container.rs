use minecraft_net_proc::Packet;

Packet!(CloseContainer, 0x12, {
    window_id: UByte,
});