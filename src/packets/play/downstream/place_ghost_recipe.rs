use minecraft_net_proc::Packet;

#[derive(Packet)]
#[id = 0x39]
pub struct PlaceGhostRecipe {
    window_id: i8,
    recipe: String,
}
impl PlaceGhostRecipe {
    pub fn new(window_id: i8, recipe: String) -> Self {
        Self {window_id, recipe}
    }
}