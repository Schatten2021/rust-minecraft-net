use minecraft_net_proc::Packet;

Packet!(SetContainerProperty, 0x14, {
    window_id: UByte,
    property: Short,
    value: Short,
});