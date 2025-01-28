use minecraft_net_proc::{Packet, Packet_old};
use crate::fields::types::{PrefixedOptional, UByte};

Packet!(StoreCookie, 0x0A, {
    payload: PrefixedOptional<UByte>
});