use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x05]
pub struct AcknowledgeBlockChange {
    #[Var]
    pub sequence_id: i32,
}
impl AcknowledgeBlockChange {
    pub fn new(sequence_id: i32) -> Self {
        Self { sequence_id }
    }
}