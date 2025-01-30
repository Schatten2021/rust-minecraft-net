use crate::fields::encode_var_int;
use crate::fields::types::{TextComponent, VarInt};
use crate::packets::BitSet;
use crate::{Field, PacketReader};
use minecraft_net_proc::{Field, Packet, VarIntEnum};

#[derive(Clone, Debug)]
struct Signature {
    dat: Vec<u8>,
}
impl Field for Signature {
    fn to_bytes(&self) -> Vec<u8> {
        self.dat.clone()
    }
    fn from_reader(reader: &mut PacketReader) -> crate::errors::Result<Self> {
        Ok(Self {
            dat: reader.read_byte_array(256),
        })
    }
}

Field!(Header, {
    sender: UUID,
    index: VarInt,
    signature: PrefixedOptional<Signature>
});
Field!(Body, {
    message: String,
    timestamp: Long,
    salt: Long,
});
#[derive(Clone, Debug)]
enum PreviousMessage {
    ID(VarInt),
    Signature(Signature),
}
impl Field for PreviousMessage {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::ID(id) => encode_var_int(id + 1),
            Self::Signature(signature) => signature.to_bytes(),
        }
    }
    fn from_reader(reader: &mut PacketReader) -> crate::errors::Result<Self> {
        let id = reader.read_var_int()?;
        if id == 0 {
            Ok(Self::Signature(Signature::from_reader(reader)?))
        } else {
            Ok(Self::ID(id - 1))
        }
    }
}
VarIntEnum!(FilterType, {
    Passthrough,
    FullyFiltered,
    PartiallyFiltered: BitSet,
});
Field!(Other, {
    unsigned_content: PrefixedOptional<TextComponent>,
    filter_type: FilterType,
});
Field!(ChatFormatting, {
    chat_type: VarInt,
    sender_name: TextComponent,
    target_name: PrefixedOptional<TextComponent>,
});

Packet!(PlayerChatMessage, 0x3B, {
    header: Header,
    body: Body,
    previous_messages: PrefixedArray<PreviousMessage>,
    other: Other,
    chat_formatting: ChatFormatting,
});