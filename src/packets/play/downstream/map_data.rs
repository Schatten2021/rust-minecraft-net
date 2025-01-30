use minecraft_net_proc::{Field, Field_old, Packet};
use crate::{Field, PacketReader};
use crate::fields::types::{PrefixedArray, PrefixedOptional, TextComponent, UByte};
Field!(Icon, {
    r#type: VarInt,
    x: Byte,
    z: Byte,
    direction: Byte,
    display_name: PrefixedOptional<TextComponent>,
});
#[derive(Debug, Clone)]
pub struct ColourPatch {
    columns: u8,
    data: Option<ColourPatchData>
}
Field!(ColourPatchData, {
    rows: UByte,
    x: UByte,
    z: UByte,
    data: PrefixedArray<UByte>
});
impl Field for ColourPatch {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = vec![self.columns];
        if self.columns > 0 {
            res.append(&mut self.data.as_ref().expect("more than 0 rows but no data").to_bytes());
        }
        res
    }
    fn from_reader(reader: &mut PacketReader) -> crate::errors::Result<Self> {
        let columns = reader.read_ubyte();
        if columns > 0 {
            Ok(Self {
                columns,
                data: Some(ColourPatchData::from_reader(reader)?),
            })
        } else {
            Ok(Self {
                columns, data: None
            })
        }
    }
}

Packet!(MapData, 0x2D, {
    map_id: VarInt,
    scale: Byte,
    locked: bool,
    icons: PrefixedOptional<PrefixedArray<Icon>>,
    color_patch: ColourPatch,
});