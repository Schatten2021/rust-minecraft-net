use crate::{Packet, PacketReader, Result};
use crate::fields::{encode_angle, encode_double, encode_short, encode_uuid, encode_var_int};

#[derive(Debug)]
pub struct  SpawnEntity {
    entity_id: i32,
    entity_uuid: u128,
    r#type: i32,
    x: f64,
    y: f64,
    z: f64,
    pitch: u8,
    yaw: u8,
    head_yaw: u8,
    data: i32,
    vel_x: i16,
    vel_y: i16,
    vel_z: i16,
}

impl Packet for SpawnEntity {
    const ID: i32 = 0x02;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_var_int(self.entity_id),
            encode_uuid(self.entity_uuid),
            encode_var_int(self.r#type),
            encode_double(self.x),
            encode_double(self.y),
            encode_double(self.z),
            encode_angle(self.pitch),
            encode_angle(self.yaw),
            encode_angle(self.head_yaw),
            encode_var_int(self.data),
            encode_short(self.vel_x),
            encode_short(self.vel_y),
            encode_short(self.vel_z),
        ].iter().flatten().copied().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            entity_id: reader.read_var_int()?,
            entity_uuid: reader.read_uuid(),
            r#type: reader.read_var_int()?,
            x: reader.read_double(),
            y: reader.read_double(),
            z: reader.read_double(),
            pitch: reader.read_angle(),
            yaw: reader.read_angle(),
            head_yaw: reader.read_angle(),
            data: reader.read_var_int()?,
            vel_x: reader.read_short(),
            vel_y: reader.read_short(),
            vel_z: reader.read_short(),
        })
    }
}
impl SpawnEntity {
    pub fn new(entity_id: i32, entity_uuid: u128, r#type: i32, x: f64, y: f64, z: f64, pitch: u8, yaw: u8, head_yaw: u8, data: i32, vel_x: i16, vel_y: i16, vel_z: i16) -> Self {
        Self {entity_id, entity_uuid, r#type, x, y, z, pitch, yaw, head_yaw, data, vel_x, vel_y, vel_z}
    }
}