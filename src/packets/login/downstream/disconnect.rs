use crate::fields::types::TextComponent;
use minecraft_net_proc::Packet;

Packet!(Disconnect, 0x00, {
    reason: TextComponent,
});