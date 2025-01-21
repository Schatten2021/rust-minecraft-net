use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x27]
pub struct ClientBoundKeepAlive {
    #[Const]
    pub keep_alive_id: i64,
}
impl ClientBoundKeepAlive {
    pub fn new(keep_alive_id: i64) -> Self {Self {keep_alive_id}}
}