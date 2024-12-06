use minecraft_net_proc::Packet;
#[derive(Debug, Packet)]
#[id = 0x04]
pub struct ClientBoundKeepAlive {
    #[Const]
    pub id: i64,
}
impl ClientBoundKeepAlive {
    pub fn new(id: i64) -> Self {
        Self {id}
    }
}