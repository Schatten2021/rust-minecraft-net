use minecraft_net_proc::Packet;

Packet!(CookieResponse, 0x04, {
    key: String,
    payload: PrefixedOptional<PrefixedArray<UByte>>
});