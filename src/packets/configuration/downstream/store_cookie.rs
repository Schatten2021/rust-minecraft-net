use minecraft_net_proc::Packet;

Packet!(StoreCookie, 0x0A, {
    key: Identifier,
    payload: PrefixedArray<UByte>
});