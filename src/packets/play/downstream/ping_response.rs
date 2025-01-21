use minecraft_net_proc::Packet;

#[derive(Packet)]
#[id = 0x38]
pub struct PingResponse {
    #[Const]
    pub payload: i64
}