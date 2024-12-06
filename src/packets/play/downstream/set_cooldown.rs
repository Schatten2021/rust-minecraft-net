use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x17]
pub struct SetCooldown {
    #[Var]
    pub item_id: i32,
    #[Var]
    pub cooldown_ticks: i32,
}
impl SetCooldown {
    pub fn new(item_id: i32, cooldown_ticks: i32) -> Self {
        Self {item_id, cooldown_ticks}
    }
}