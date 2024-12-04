use crate::{Packet, PacketReader, Result};
use crate::fields::{encode_double, encode_var_int, encode_var_long};

#[derive(Debug)]
pub struct InitializeWorldBorder {
    pub x: f64,
    pub y: f64,
    pub old_diameter: f64,
    pub new_diameter: f64,
    pub speed: i64,
    pub portal_teleport_boundary: i32,
    pub warning_blocks: i32,
    pub warning_time: i32,
}

impl Packet for InitializeWorldBorder {
    const ID: i32 = 0x25;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_double(self.x),
            encode_double(self.y),
            encode_double(self.old_diameter),
            encode_double(self.new_diameter),
            encode_var_long(self.speed),
            encode_var_int(self.portal_teleport_boundary),
            encode_var_int(self.warning_blocks),
            encode_var_int(self.warning_time),
        ].iter().flatten().cloned().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            x: reader.read_double(),
            y: reader.read_double(),
            old_diameter: reader.read_double(),
            new_diameter: reader.read_double(),
            speed: reader.read_var_long()?,
            portal_teleport_boundary: reader.read_var_int()?,
            warning_blocks: reader.read_var_int()?,
            warning_time: reader.read_var_int()?,
        })
    }
}
impl InitializeWorldBorder {
    pub fn new(x: f64, y: f64, old_diameter: f64, new_diameter: f64, speed: i64, portal_teleport_boundary: i32, warning_blocks: i32, warning_time: i32) -> Self {
        Self {x, y, old_diameter, new_diameter, speed, portal_teleport_boundary, warning_blocks, warning_time}
    }
}