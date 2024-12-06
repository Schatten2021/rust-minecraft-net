use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x01]
pub struct  SpawnEntity {
    #[Var]
    entity_id: i32,
    entity_uuid: u128,
    #[Var]
    r#type: i32,
    x: f64,
    y: f64,
    z: f64,
    pitch: u8,
    yaw: u8,
    head_yaw: u8,
    #[Var]
    data: i32,
    vel_x: i16,
    vel_y: i16,
    vel_z: i16,
}
impl SpawnEntity {
    pub fn new(entity_id: i32, entity_uuid: u128, r#type: i32, x: f64, y: f64, z: f64, pitch: u8, yaw: u8, head_yaw: u8, data: i32, vel_x: i16, vel_y: i16, vel_z: i16) -> Self {
        Self {entity_id, entity_uuid, r#type, x, y, z, pitch, yaw, head_yaw, data, vel_x, vel_y, vel_z}
    }
}