use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x1A]
pub struct DamageEvent {
    #[Var]
    pub entity_id: i32,
    #[Var]
    pub source_type_id: i32,
    #[Var]
    pub source_cause_id: i32,
    #[Var]
    pub source_direct_id: i32,
    pub has_source_position: bool,
    #[when = "has_source_position"]
    pub source_position_x: Option<f64>,
    #[when = "has_source_position"]
    pub source_position_y: Option<f64>,
    #[when = "has_source_position"]
    pub source_position_z: Option<f64>,
}
impl DamageEvent {
    pub fn new(entity_id: i32,
               source_type_id: i32,
               source_cause_id: i32,
               source_direct_id: i32,
               has_source_position: bool,
               source_position_x: Option<f64>,
               source_position_y: Option<f64>,
               source_position_z: Option<f64>,) -> Self {
        Self {entity_id, source_type_id, source_cause_id, source_direct_id, has_source_position, source_position_x, source_position_y, source_position_z, }
    }
}