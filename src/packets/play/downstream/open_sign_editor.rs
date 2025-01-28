use crate::packets::Position;
use minecraft_net_proc::Packet;

Packet!(OpenSignEditor, 0x36, {
    location: Position,
    is_front_text: bool,
});