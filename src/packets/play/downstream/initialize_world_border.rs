use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x25]
pub struct InitializeWorldBorder {
    pub x: f64,
    pub y: f64,
    pub old_diameter: f64,
    pub new_diameter: f64,
    #[Var]
    pub speed: i64,
    #[Var]
    pub portal_teleport_boundary: i32,
    #[Var]
    pub warning_blocks: i32,
    #[Var]
    pub warning_time: i32,
}

impl InitializeWorldBorder {
    pub fn new(x: f64, y: f64, old_diameter: f64, new_diameter: f64, speed: i64, portal_teleport_boundary: i32, warning_blocks: i32, warning_time: i32) -> Self {
        Self {x, y, old_diameter, new_diameter, speed, portal_teleport_boundary, warning_blocks, warning_time}
    }
}