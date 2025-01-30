use minecraft_net_proc::{Field, Packet};
use crate::fields::slot::{Component, Slot};

Field!(TradeItem, {
    item_id: VarInt,
    item_count: VarInt,
    components: PrefixedArray<Component>,
});

Field!(Trade, {
    input_item1: TradeItem,
    output_item: Slot,
    input_item2: PrefixedOptional<TradeItem>,
    trade_disabled: bool,
    number_of_trade_uses: Int,
    maximum_number_of_trade_uses: Int,
    xp: Int,
    special_price: Int,
    price_multiplier: Float,
    demand: Int,
});
Packet!(MerchantOffers, 0x2E, {
    window_id: VarInt,
    trades: PrefixedArray<Trade>,
});