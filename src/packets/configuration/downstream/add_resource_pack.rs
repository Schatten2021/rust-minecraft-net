use minecraft_net_proc::Packet;
#[derive(Debug, Packet)]
#[id = 0x09]
pub struct AddResourcePack {
    pub uuid: u128,
    pub url: String,
    pub hash: String,
    pub forced: bool,
    pub has_prompt_message: bool,
    #[when = "has_prompt_message"]
    pub prompt_message: Option<String>,
}
impl AddResourcePack {
    pub fn new(uuid: u128, url: String, hash: String, forced: bool, prompt_message: Option<String>) -> Self {
        Self {
            uuid, url, hash, forced,
            has_prompt_message: prompt_message.is_some(),
            prompt_message,
        }
    }
}