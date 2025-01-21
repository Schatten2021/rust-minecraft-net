use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x33]
pub struct MoveVehicle {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}