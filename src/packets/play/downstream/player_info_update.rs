// TODO
// use minecraft_net_proc::Field;
// use crate::fields::types::{PrefixedArray, PrefixedOptional, VarInt, UUID};
// Field!(Property, {
//     name: String,
//     value: String,
//     signature: PrefixedOptional<String>,
// });
// Field!(AddPlayerAction, {
//     name: String,
//     properties: PrefixedArray<Property>,
// });
// 
// Field!(UserKeyData, {
//     public_key_expiry_time: Long,
//     encoded_public_key: PrefixedArray<Byte>,
//     public_key_signature: PrefixedArray<Byte>,
// });
// Field!(InitializeChatAction, {
//     chat_session_id: UUID,
//     key_data: PrefixedOptional<UserKeyData>,
// });
// 
// pub enum PlayerAction {
//     AddPlayer(AddPlayerAction),
//     InitializeChat(InitializeChatAction),
//     UpdateGameMode(VarInt),
//     UpdateListed(bool),
//     UpdateLatency(VarInt),
//     UpdateDisplayName(PrefixedOptional<String>),
//     UpdateListPriority(VarInt),
//     UpdateHat(bool),
// }
// impl PlayerAction {
//     pub fn to_bytes(&self) -> Vec<u8> {
//         match self {
//             PlayerAction::AddPlayer(d) => {}
//             PlayerAction::InitializeChat(d) => {}
//             PlayerAction::UpdateGameMode(d) => {}
//             PlayerAction::UpdateListed(d) => {}
//             PlayerAction::UpdateLatency(d) => {}
//             PlayerAction::UpdateDisplayName(d) => {}
//             PlayerAction::UpdateListPriority(d) => {}
//             PlayerAction::UpdateHat(d) => {}
//         }
//     }
// }
// 
// 
// pub struct Player {
//     pub uuid: UUID,
//     pub actions: Vec<PlayerAction>
// }
// // use minecraft_net_proc::{Field_old, Packet_old};
// // use crate::fields::types::{PrefixedArray, UUID};
// // TODO
// // #[derive(Clone, Debug, Field_old)]
// // pub struct Player {
// //     pub uuid: UUID,
// //     // pub player_actions: Vec<PlayerActon> TODO: figure out
// // }
// // 
// // #[derive(Debug, Packet_old)]
// // #[id = 0x40]
// // pub struct PlayerInfoUpdate {
// //     // actions: EnumSet TODO: figure out
// //     players: PrefixedArray<Player>
// // }