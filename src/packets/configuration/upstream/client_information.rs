use crate::fields::{encode_bool, encode_byte, encode_string, encode_ubyte, encode_var_int};
use crate::{Packet, PacketReader, Result};

#[derive(Debug)]
pub struct ClientInformation {
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: i32,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: i32,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
}
impl Packet for ClientInformation {
    const ID: i32 = 0x00;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_string(self.locale.clone());
        res.append(&mut encode_byte(self.view_distance));
        res.append(&mut encode_var_int(self.chat_mode));
        res.append(&mut encode_bool(self.chat_colors));
        res.append(&mut encode_ubyte(self.displayed_skin_parts));
        res.append(&mut encode_var_int(self.main_hand));
        res.append(&mut encode_bool(self.enable_text_filtering));
        res.append(&mut encode_bool(self.allow_server_listings));
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            locale: reader.read_string()?,
            view_distance: reader.read_byte(),
            chat_mode: reader.read_var_int()?,
            chat_colors: reader.read_bool()?,
            displayed_skin_parts: reader.read_ubyte(),
            main_hand: reader.read_var_int()?,
            enable_text_filtering: reader.read_bool()?,
            allow_server_listings: reader.read_bool()?,
        })
    }
}
impl ClientInformation {
    pub fn new(locale: String, view_distance: i8, chat_mode: i32, chat_colors: bool, displayed_skin_parts: u8, main_hand: i32, enable_text_filtering: bool, allow_server_listings: bool) -> Self {
        Self {locale, view_distance, chat_mode, chat_colors, displayed_skin_parts, main_hand, enable_text_filtering, allow_server_listings}
    }
}