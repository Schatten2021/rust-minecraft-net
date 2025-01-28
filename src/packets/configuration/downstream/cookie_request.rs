use minecraft_net_proc::Packet;
Packet!(CookieRequest, 0x00, {
    key: String,
});