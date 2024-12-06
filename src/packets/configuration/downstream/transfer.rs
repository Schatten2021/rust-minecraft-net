use minecraft_net_proc::Packet;
#[derive(Debug, Packet)]
#[id = 0x0B]
pub struct Transfer {
    pub host: String,
    #[Var]
    pub port: i32,
}
impl Transfer {
    pub  fn new(host: String, port: i32) -> Self {
        Self{host, port}
    }
}