use minecraft_net_proc::{Packet, Packet_old};
use crate::fields::types::TextComponent;

Packet!(Disconnect, 0x00, {
    reason: TextComponent,
});