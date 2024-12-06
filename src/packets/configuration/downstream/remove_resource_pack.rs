use minecraft_net_proc::Packet;
#[derive(Debug, Packet)]
#[id = 0x08]
pub struct RemoveResourcePack {
    pub has_uuid: bool,
    #[when = "has_uuid"]
    pub uuid: Option<u128>,
}
impl RemoveResourcePack {
    pub fn new(uuid: Option<u128>) -> Self {
        Self {
            has_uuid: uuid.is_some(),
            uuid,
        }
    }
}