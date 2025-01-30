use minecraft_net_proc::Packet;
use crate::fields::types::TextComponent;

Packet!(Disconect, 0x1D, {
    reason: TextComponent,
});