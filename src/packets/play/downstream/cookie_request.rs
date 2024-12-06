use minecraft_net_proc::Packet;
#[derive(Debug, Packet)]
#[id = 0x16]
pub struct CookieRequest {
    pub key: String
}
impl CookieRequest {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}