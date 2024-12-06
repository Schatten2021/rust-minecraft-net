use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x14]
pub struct SetContainerProperty {
    pub window_id: u8,
    pub property: i16,
    pub value: i16,
}

impl SetContainerProperty {
    pub fn new(window_id: u8, property: i16, value: i16) -> Self {
        Self {window_id, property, value}
    }
}