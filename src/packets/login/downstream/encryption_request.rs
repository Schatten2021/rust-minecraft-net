use minecraft_net_proc::Packet;
Packet!(EncryptionRequest, 0x01, {
    server_id: String,
    public_key: PrefixedArray<UByte>,
    verify_token: PrefixedArray<UByte>,
    should_authenticate: bool,
});