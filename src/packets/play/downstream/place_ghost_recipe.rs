// use minecraft_net_proc::{Field, Packet, VarIntEnum};
// 
// Field!(CraftingShapeless, {
//     ingredient_count: VarInt,
//     ingredients: Vec<SlotDisplay>
// })
// 
// VarIntEnum!(RecipeDisplay, {
//     CraftingShapeless,
//     CraftingShaped,
//     Furnnace,
//     StoneCutter,
//     Smithing,
// });
// 
// Packet!(PlaceGhostRecipe, 0x39, {
//     window_id: VarInt,
//     recipe_display: RecipeDisplay
// });
// TODO: This changed. Update.