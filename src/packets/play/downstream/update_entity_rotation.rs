use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x32]
pub struct UpdateEntityRotation {
    #[Var]
    pub entity_id: i32,
    pub yaw: u8,
    pub pitch: u8,
    pub on_ground: bool,
}