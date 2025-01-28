use minecraft_net_proc::Packet;

Packet!(EncryptionResponse, 0x01, {
    shared_secret: PrefixedArray<UByte>,
    verify_token: PrefixedArray<UByte>,
});