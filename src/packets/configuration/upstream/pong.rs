use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x05]
pub struct Pong {
    #[Const]
    pub id: i32
}
impl Pong {
    pub fn new(id: i32) -> Self {
        Self {id}
    }
}