use minecraft_net_proc::Packet;
Packet!(CookieRequest, 0x05, {
    key: String,
});