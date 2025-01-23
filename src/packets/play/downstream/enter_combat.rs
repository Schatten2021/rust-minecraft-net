use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x3D]
pub struct EnterCombat {}