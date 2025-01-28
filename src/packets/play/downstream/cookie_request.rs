use minecraft_net_proc::Packet;
Packet!(CookieRequest, 0x16, {
    key: String,
});