use crate::fields::block_predicate::BlockPredicate;
use crate::fields::general::IDSet;
use crate::fields::types::*;
use crate::fields::{encode_bool, encode_prefixed_array, encode_var_int};
use crate::PacketReader;
use crate::{Field, Result};
use minecraft_net_proc::{Field, Field_old, VarIntEnum};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Slot {
    pub item_count: VarInt,
    pub data: Option<SlotData>,
}
impl Field for Slot {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_var_int(self.item_count);
        if self.item_count > 0 {
            res.append(&mut self.data.clone().expect("Slot item count bigger than 0 but no data was provided.").to_bytes());
        }
        res
    }

    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let count = reader.read_var_int()?;
        if count > 0 {
            Ok(Self {
                item_count: count,
                data: Some(SlotData::from_reader(reader)?),
            })
        } else {
            Ok(Self {
                item_count: count,
                data: None,
            })
        }
    }
}
#[derive(Debug, Clone, Field_old)]
pub struct SlotData {
    pub item_id: VarInt,
    pub number_of_components_to_add: VarInt,
    pub number_of_components_to_remove: VarInt,
    #[len = "number_of_components_to_add"]
    pub components_to_add: Vec<Component>,
    #[len = "number_of_components_to_remove"]
    pub components_to_remove: Vec<VarInt>,
}
VarIntEnum!(Component, {
        CustomData: NBT,
        MaxStackSize: VarInt,
        MaxDamage: VarInt,
        Damage: VarInt,
        Unbreakable: bool,
        CustomName: String,
        ItemName: String,
        ItemModel: Identifier,
        Lore: PrefixedArray<String>,
        Rarity: VarInt,
        Enchantments: OptionalTooltip<PrefixedArray<Enchantment>>,
        CanPlaceOn: OptionalTooltip<PrefixedArray<BlockPredicate>>,
        CanBreak: OptionalTooltip<PrefixedArray<BlockPredicate>>,
        AttributeModifiers: OptionalTooltip<PrefixedArray<AttributeModifier>>,
        CustomModelData: CustomModelData,
        HideAdditionalTooltip,
        HideTooltip,
        RepairCost: VarInt,
        CreativeSlotLock,
        EnchantmentGlintOverride: bool,
        IntangibleProjectile: NBT,
        Food: Food,
        Consumable: Consumable,
        UseRemainder: Slot,
        UseCooldown: UseCooldown,
        DamageResistant: Identifier,
        Tool: Tool,
        Enchantable: VarInt,
        Equippable: Equippable,
        Reparable: IDSet,
        Glider,
        TooltipStyle: Identifier,
        // DeathProtection: PrefixedArray<ConsumeEffect> TODO
        StoredEnchantments: OptionalTooltip<StoredEnchantment>,
        DyedColor: DyedColor,
        MapColor: Int,
        MapId: VarInt,
        MapDecorations: NBT,
        MapPostProcessing: VarInt,
        ChargedProjectiles: PrefixedArray<Slot>,
        BundleContents: PrefixedArray<Slot>,
        PotionContents: PotionContents,
        SuspiciousStewEffects: PrefixedArray<SuspiciousStewEffect>,
        WritableBookContent: WritableBookContent,
        WrittenBookContent: WrittenBookContent,
        Trim: Trim,
        DebugStickState: NBT,
        EntityData: NBT,
        BucketEntityData: NBT,
        BlockEntityData: NBT,
        // Instrument: IdOr<Instrument>, TODO
        OminousBottleAmplifier: VarInt,
        JukeboxPlayable: JukeboxPlayable,
        Recipes: NBT,
        LodestoneTracker: LodestoneTracker,
        // FireworkExplosion: FireworkExplosion, TODO
        Profile: Profile,
        NoteBlockSound: Identifier,
        BannerPatterns: PrefixedArray<BannerPattern>,
        // BaseColor: DyeColor, TODO
        PotDecorations: PrefixedArray<VarInt>,
        Container: PrefixedArray<Slot>,
        BlockState: PrefixedArray<BlockState>,
        Bees: PrefixedArray<Bee>,
        Lock: NBT,
        ContainerLoot: NBT,
    });
#[derive(Debug, Clone)]
pub struct OptionalTooltip<T: Debug + Clone> {
    pub value: T,
    pub show_tooltip: bool,
}
impl<T: Debug + Clone + Field> Field for OptionalTooltip<T> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = self.value.to_bytes();
        res.append(&mut encode_bool(self.show_tooltip));
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            value: T::from_reader(reader)?,
            show_tooltip: reader.read_bool()?,
        })
    }
}
impl<T: Debug + Clone + Field> Field for OptionalTooltip<PrefixedArray<T>> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_prefixed_array(&self.value);
        res.append(&mut encode_bool(self.show_tooltip));
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            value: reader.read_prefixed_array()?,
            show_tooltip: reader.read_bool()?,
        })
    }
}
#[derive(Debug, Clone, Field_old)]
pub struct Enchantment {
    pub type_id: VarInt,
    pub level: VarInt,
}
Field!(AttributeModifier, {
        attribute_id: VarInt,
        modifier_id: Identifier,
        value: Double,
        operation: VarInt,
        slot: VarInt,
    });
Field!(CustomModelData, {
        floats: PrefixedArray<Float>,
        flags: PrefixedArray<bool>,
        strings: PrefixedArray<String>,
        colors: PrefixedArray<Int>,
    });
#[derive(Clone, Debug, Field_old)]
pub struct Food {
    pub nutrition: VarInt,
    pub saturation_modifier: Float,
    pub can_always_eat: bool,
}
#[derive(Clone, Debug, Field_old)]
pub struct Consumable {
    pub consume_seconds: Float,
    pub animation: VarInt,
    // pub sound: IdOR<SoundEvent>, TODO
    pub has_consume_particles: bool,
    // pub effects: PrefixedArray<ConsumeEffect>, TODO
}
#[derive(Clone, Debug, Field_old)]
pub struct UseCooldown {
    pub seconds: Float,
    // TODO: verify that this is true.
    // The wiki says something about "Only present if Has cooldown group is true" but doesn't mention "Has Cooldown group" anywhere else.
    pub cooldown_group: PrefixedOptional<Identifier>
}
#[derive(Clone, Debug, Field_old)]
pub struct Tool {
    pub rule: PrefixedArray<ToolRule>,
    pub default_mining_speed: Float,
    pub damage_per_block: VarInt,
}
#[derive(Clone, Debug, Field_old)]
pub struct ToolRule {
    pub blocks: IDSet,
    pub speed: PrefixedOptional<Float>,
    pub correct_drop_for_blocks: PrefixedOptional<bool>,
}
#[derive(Clone, Debug, Field_old)]
pub struct Equippable {
    pub slot: VarInt,
    // pub EquipSound: IdOr<SoundEvent>, TODO
    pub model: PrefixedOptional<Identifier>,
    pub camera_overlay: PrefixedOptional<Identifier>,
    pub allowed_entities: PrefixedOptional<IDSet>,
    pub dispensable: bool,
    pub swappable: bool,
    pub damage_on_hurt: bool,
}
#[derive(Clone, Debug, Field_old)]
pub struct StoredEnchantment {
    pub type_id: VarInt,
    pub level: VarInt,
}
#[derive(Clone, Debug, Field_old)]
pub struct DyedColor {
    pub color: Int,
    pub show_in_tooltip: bool,
}
#[derive(Clone, Debug, Field_old)]
pub struct PotionContents {
    pub potion_id: PrefixedOptional<VarInt>,
    pub custom_color: PrefixedOptional<Int>,
    // pub custom_effects: PrefixedArray<PotionEffect>, TODO
    pub custon_name: String,
}
#[derive(Clone, Debug, Field_old)]
pub struct SuspiciousStewEffect {
    pub type_id: VarInt,
    pub duration: VarInt,
}
#[derive(Clone, Debug, Field_old)]
pub struct WritableBookContent {
    pub raw_content: String,
    pub filtered_content: PrefixedOptional<String>,
}
#[derive(Clone, Debug, Field_old)]
pub struct WrittenBookContent {
    pub raw_title: String,
    pub filtered_title: PrefixedOptional<String>,
    pub author: String,
    pub generation: VarInt,
    pub pages: PrefixedArray<WritableBookContent>,
    pub resolved: bool,
}
#[derive(Clone, Debug, Field_old)]
pub struct Trim {
    // pub trim_material: IdOr<TrimMaterial>, TODO
    // pub trim_pattern: IdOr<TrimPattern>, TODO
    pub show_in_tooltip: bool
}
#[derive(Clone, Debug)]
pub struct JukeboxPlayable {
    pub direct_mode: bool,
    pub jukebox_song_name: Option<Identifier>,
    // pub jukebox_song: Option<IdOr<JukeboxSong>>, TODO
    pub show_in_tooltip: bool,
}
impl Field for JukeboxPlayable {
    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        todo!()
    }
}
#[derive(Clone, Debug, Field_old)]
pub struct LodestoneTracker {
    pub has_global_position: bool,
    pub dimension: Identifier,
    pub position: Position,
    pub tracked: bool,
}
#[derive(Clone, Debug, Field_old)]
pub struct Fireworks {
    pub flight_duration: VarInt,
    // pub Explosions: PrefixedArray<FireworkExplosion>, TODO
}
#[derive(Clone, Debug, Field_old)]
pub struct Profile {
    pub name: PrefixedOptional<String>,
    pub uuid: PrefixedOptional<UUID>,
    pub property: PrefixedArray<ProfileProperty>
}
#[derive(Clone, Debug, Field_old)]
pub struct ProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: PrefixedOptional<String>,
}
#[derive(Clone, Debug)]
pub struct BannerPattern {
    pub pattern_type: VarInt,
    pub asset_id: Option<Identifier>,
    pub translation_key: Option<String>,
    // pub color: DyeColor, TODO
}
impl Field for BannerPattern {
    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        todo!()
    }
}
#[derive(Clone, Debug, Field_old)]
pub struct BlockState {
    pub name: String,
    pub value: String,
}
#[derive(Clone, Debug, Field_old)]
pub struct Bee {
    pub entity_data: NBT,
    pub ticks_in_hive: VarInt,
    pub min_ticks_in_hive: VarInt,
}
