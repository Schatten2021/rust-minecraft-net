use minecraft_net_proc::{Packet, Packet_old};
Packet!(CookieRequest, 0x05, {
    key: String,
});