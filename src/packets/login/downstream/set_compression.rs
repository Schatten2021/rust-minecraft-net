use minecraft_net_proc::Packet;
#[derive(Debug, Packet)]
#[id = 0x03]
pub struct SetCompression {
    #[Var]
    pub threshold: i32,
}
impl SetCompression {
    pub fn new(threshold: i32) -> Self {
        Self { threshold }
    }
}