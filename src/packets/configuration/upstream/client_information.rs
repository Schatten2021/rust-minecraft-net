use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x00]
pub struct ClientInformation {
    pub locale: String,
    pub view_distance: i8,
    #[Var]
    pub chat_mode: i32,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    #[Var]
    pub main_hand: i32,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
}
impl ClientInformation {
    pub fn new(locale: String, view_distance: i8, chat_mode: i32, chat_colors: bool, displayed_skin_parts: u8, main_hand: i32, enable_text_filtering: bool, allow_server_listings: bool) -> Self {
        Self {locale, view_distance, chat_mode, chat_colors, displayed_skin_parts, main_hand, enable_text_filtering, allow_server_listings}
    }
}