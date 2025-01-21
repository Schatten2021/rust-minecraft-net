use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x24]
pub struct OpenHorseScreen {
    window_id: u8,
    #[Var]
    slot_count: i32,
    #[Const]
    entity_id: i32,
}

impl OpenHorseScreen {
    pub fn new(window_id: u8, slot_count: i32, entity_id: i32) -> Self {
        Self {window_id, slot_count, entity_id}
    }
}