use crate::packets::configuration::KnownPack;
use minecraft_net_proc::Packet;
Packet!(ClientBoundKnownPacks, 0x0E, {
    known_packs: PrefixedArray<KnownPack>,
});