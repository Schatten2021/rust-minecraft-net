use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x06]
pub struct ResourcePackResponse {
    pub uuid: u128,
    #[Var]
    pub result: i32,
}

impl ResourcePackResponse {
    pub fn new(uuid: u128, result: i32) -> Self {
        Self {uuid, result}
    }
}