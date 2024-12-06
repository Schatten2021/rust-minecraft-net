use minecraft_net_proc::Packet;
use rand::random;
#[derive(Debug, Packet)]
#[id = 0x05]
pub struct Ping {
    #[Const]
    pub id: i32,
}
impl Ping {
    pub fn new() -> Self {
        Self{id: random()}
    }
}