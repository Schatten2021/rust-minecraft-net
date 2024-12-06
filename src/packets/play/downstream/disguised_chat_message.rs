use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x1E]
pub struct DisguisedChatMessage {
    pub message: String,
    #[Var]
    pub chat_type: i32,
    pub sender_name: String,
    pub has_target: bool,
    pub target: String, //TODO: maybe Option<String>?
}
impl DisguisedChatMessage {
    pub fn new(message: String, chat_type: i32, sender_name: String, has_target: bool, target: String) -> Self {
        Self {message, chat_type, sender_name, has_target, target}
    }
}