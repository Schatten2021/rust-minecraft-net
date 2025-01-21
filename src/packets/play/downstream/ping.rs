use minecraft_net_proc::Packet;

#[derive(Packet)]
#[id = 0x37]
pub struct Ping {
    #[Const]
    pub id: i32
}