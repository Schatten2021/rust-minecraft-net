use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x22]
pub struct GameEvent {
    event: u8,
    value: f32,
}
impl GameEvent {
    pub fn new(event: u8, value: f32) -> Self {
        Self { event, value }
    }
}