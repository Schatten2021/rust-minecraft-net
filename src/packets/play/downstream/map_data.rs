use minecraft_net_proc::Field;
use crate::fields::types::{PrefixedArray, PrefixedOptional};

#[derive(Debug, Field, Clone)]
pub struct Icon {
    #[Var]
    r#type: i32,
    x: i8,
    z: i8,
    direction: i8,
    has_display_name: bool,
    #[when = "has_display_name"]
    display_name: Option<String>
}
impl Icon {
    pub fn new(r#type: i32, x: i8, z: i8, direction: i8, display_name: Option<String>) -> Self {
        Self {
            r#type, x, z, direction, has_display_name: display_name.is_some(), display_name
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColourPatch {
    columns: u8,
    rows: Option<u8>,
    x: Option<u8>,
    z: Option<u8>,
    length: Option<i32>,
    data: Option<Vec<u8>> //TODO: figure out data type
}

#[derive(Debug)]
// #[id = 0x2D]
pub struct MapData {
    map_id: i32,
    scale: u8,
    locked: bool,
    has_icons: bool,
    icons: PrefixedOptional<PrefixedArray<Icon>>,
    
}