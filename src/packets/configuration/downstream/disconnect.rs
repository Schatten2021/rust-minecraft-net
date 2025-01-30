use minecraft_net_proc::Packet;
use crate::fields::types::TextComponent;

Packet!(Disconnect, 0x02, {
    reason: TextComponent,
});