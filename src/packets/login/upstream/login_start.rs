use minecraft_net_proc::Packet;
#[derive(Debug, Packet)]
#[id = 0x00]
pub struct LoginStart {
    pub name: String,
    pub uuid: u128,
}
impl LoginStart {
    pub fn new(name: String, uuid: u128) -> Self {
        Self {name, uuid}
    }
}