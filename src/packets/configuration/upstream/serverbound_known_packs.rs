use crate::packets::configuration::KnownPack;
use minecraft_net_proc::Packet;

Packet!(ServerBoundKnownPacks, 0x07, {
    kown_packs: PrefixedArray<KnownPack>
});