use minecraft_net_proc::Packet;

// #[derive(Debug, Packet)]
// #[id = 0x3B]
// pub struct PlayerChatMessage {
//     sender: u128,
//     #[Var]
//     index: i32,
//     message_signature_present: bool,
//     #[when = "message_signature_present"]
//     message_signature: Option<Vec<u8>>,
//     message: String,
//     timestamp: i64,
//     salt: i64,
//     #[Var]
//     total_previous_messages: i32,
// }
// impl PlayerChatMessage {
//     pub const SIGNATURE_LEN: usize = 256;
// }