use minecraft_net_proc::Packet;
use rand::random;
Packet!(Ping, 0x05, {
    id: Int,
});
impl Ping {
    pub fn random() -> Self {
        Self{id: random()}
    }
}