use minecraft_net_proc::Packet;

#[derive(Packet)]
#[id = 0x34]
pub struct OpenBook {
    #[Var]
    pub hand: i32
}
impl OpenBook {
    pub fn new(hand: i32) -> Self {
        Self { hand }
    }
}