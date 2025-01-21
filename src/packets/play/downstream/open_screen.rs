use minecraft_net_proc::Packet;

#[derive(Packet)]
#[id = 0x35]
pub struct OpenScreen {
    #[Var]
    pub window_id: i32,
    #[Var]
    pub window_type: i32,
    pub window_title: String,
}
impl OpenScreen {
    pub fn new(window_id: i32, window_type: i32, window_title: String) -> Self {
        Self {window_id, window_type, window_title}
    }
}