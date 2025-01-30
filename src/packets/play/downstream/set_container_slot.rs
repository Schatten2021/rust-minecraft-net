use crate::fields::slot::Slot;
use minecraft_net_proc::Packet;

Packet!(SetContainerSlot, 0x15, {
    window_id: VarInt,
    state_id: VarInt,
    slot: Short,
    slot_data: Slot,
});