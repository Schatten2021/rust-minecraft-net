use crate::fields::slot::Slot;
use minecraft_net_proc::Packet;

Packet!(SetContainerContent, 0x13, {
    window_id: VarInt,
    state_id: VarInt,
    slot_data: PrefixedArray<Slot>,
    carried_item: Slot,
});