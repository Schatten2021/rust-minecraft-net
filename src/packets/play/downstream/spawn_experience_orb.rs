use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x02]
pub struct SpawnExperienceOrb {
    #[Var]
    entity_id: i32,
    x: f64,
    y: f64,
    z: f64,
    count: i16,
}
impl SpawnExperienceOrb {
    pub fn new(entity_id: i32, x: f64, y: f64, z: f64, count: i16) -> Self {
        Self { entity_id, x, y, z, count }
    }
}