use minecraft_net_proc::Packet;
#[derive(Debug, Packet)]
#[id = 0x0A]
pub struct StoreCookie {
    pub key: String,
    #[Var]
    pub payload_length: i32,
    #[len = "payload_length"]
    pub payload: Vec<u8>,
}
impl StoreCookie {
    pub fn new(key: String, payload: Vec<u8>) -> Self {
        Self {
            key,
            payload_length: payload.len() as i32,
            payload
        }
    }
}