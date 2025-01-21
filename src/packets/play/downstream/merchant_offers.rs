use minecraft_net_proc::{Field, Packet};
use crate::fields::types::{PrefixedArray, VarInt};

//TODO: figure out how this stuff works
#[derive(Debug, Field, Clone)]
pub struct Component {
    #[Var]
    pub r#type: i32,
    #[Var]
    pub data: u8, //TODO: figure out type
}

#[derive(Debug, Field, Clone)]
pub struct TradeItem {
    pub item_id: VarInt,
    pub item_count: VarInt,
    pub components: PrefixedArray<Component>,
}

#[derive(Debug, Field, Clone)]
pub struct Trade {
    pub input_item1: TradeItem,
    // pub output_item: Slot TODO: figure out
    // pub input_item_2: Option<TradeItem>,
}

#[derive(Debug, Packet)]
#[id = 0x2E]
pub struct MerchantOffers {
    pub window_id: VarInt,
    pub trades: PrefixedArray<Trade>,
}