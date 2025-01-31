use crate::fields::block_predicate::BlockPredicate;
use crate::fields::general::{IDSet, IdOr};
use crate::fields::types::*;
use crate::fields::{encode_bool, encode_identifier, encode_prefixed_array, encode_string, encode_var_int};
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
        DeathProtection: PrefixedArray<ConsumeEffect>,
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
        Instrument: IdOr<Instrument>,
        OminousBottleAmplifier: VarInt,
        JukeboxPlayable: JukeboxPlayable,
        Recipes: NBT,
        LodestoneTracker: LodestoneTracker,
        FireworkExplosion: FireworkExplosion, 
        Profile: Profile,
        NoteBlockSound: Identifier,
        BannerPatterns: PrefixedArray<BannerPattern>,
        BaseColor: DyeColor, 
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
Field!(Enchantment, {
    pub type_id: VarInt,
    pub level: VarInt,
});
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
Field!(Food, {
    pub nutrition: VarInt,
    pub saturation_modifier: Float,
    pub can_always_eat: bool,
});
Field!(Consumable, {
    consume_seconds: Float,
    animation: VarInt,
    sound: IdOr<SoundEvent>,
    has_consume_particles: bool,
    effects: PrefixedArray<ConsumeEffect>,
});
Field!(UseCooldown, {
    seconds: Float,
    // TODO: verify that this is true.
    // The wiki says something about "Only present if Has cooldown group is true" but doesn't mention "Has Cooldown group" anywhere else.
    cooldown_group: PrefixedOptional<Identifier>
});
Field!(Tool, {
    rule: PrefixedArray<ToolRule>
    default_mining_speed: Float,
    damage_per_block: VarInt,
});
Field!(ToolRule, {
    pub blocks: IDSet,
    pub speed: PrefixedOptional<Float>,
    pub correct_drop_for_blocks: PrefixedOptional<bool>,
});
Field!(Equippable, {
    slot: VarInt,
    equip_sound: IdOr<SoundEvent>,
    model: PrefixedOptional<Identifier>,
    camera_overlay: PrefixedOptional<Identifier>,
    allowed_entities: PrefixedOptional<IDSet>,
    dispensable: bool,
    swappable: bool,
    damage_on_hurt: bool,
});
Field!(StoredEnchantment, {
    pub type_id: VarInt,
    pub level: VarInt,
});
Field!(DyedColor, {
    pub color: Int,
    pub show_in_tooltip: bool,
});
Field!(PotionContents, {
    pub potion_id: PrefixedOptional<VarInt>,
    pub custom_color: PrefixedOptional<Int>,
    pub custom_effects: PrefixedArray<PotionEffect>,
    pub custom_name: String,
});
Field!(SuspiciousStewEffect, {
    pub type_id: VarInt,
    pub duration: VarInt,
});
Field!(WritableBookContent, {
    pub raw_content: String,
    pub filtered_content: PrefixedOptional<String>,
});
Field!(WrittenBookContent, {
    pub raw_title: String,
    pub filtered_title: PrefixedOptional<String>,
    pub author: String,
    pub generation: VarInt,
    pub pages: PrefixedArray<WritableBookContent>,
    pub resolved: bool,
});
Field!(Trim, {
    pub trim_material: IdOr<TrimMaterial>, 
    pub trim_pattern: IdOr<TrimPattern>, 
    pub show_in_tooltip: bool
});
Field!(TrimMaterial, {
    asset_name: String,
    ingredient: VarInt,
    item_model_index: Float,
    overrides: PrefixedArray<TrimMaterialOverride>,
    description: TextComponent,
});
Field!(TrimMaterialOverride, {
    armor_material_type: VarInt,
    overriden_asset_name: String,
});
Field!(TrimPattern, {
    asset_name: String,
    template_item: VarInt,
    description: TextComponent,
    decal: bool,
});
#[derive(Clone, Debug)]
pub struct JukeboxPlayable {
    pub direct_mode: bool,
    pub jukebox_song_name: Option<Identifier>,
    pub jukebox_song: Option<IdOr<JukeboxSong>>,
    pub show_in_tooltip: bool,
}
impl Field for JukeboxPlayable {
    fn to_bytes(&self) -> Vec<u8> {
        encode_bool(self.direct_mode).into_iter()
            .chain(
                if self.direct_mode { 
                    self.jukebox_song.clone().expect("Direct mode set, but Jukebox Song is None").to_bytes()
                } else {
                    encode_identifier(self.jukebox_song_name.clone().expect("Direct mode not set, but Jukebox Song name is Non"))
                })
            .chain(encode_bool(self.show_in_tooltip))
            .collect::<Vec<u8>>()
    }

    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let direct_mode = reader.read_bool()?;
        if direct_mode {
            Ok(Self {
                direct_mode,
                jukebox_song: Some(reader.read()?),
                jukebox_song_name: None,
                show_in_tooltip: reader.read_bool()?,
            })
        } else {
            Ok(Self {
                direct_mode,
                jukebox_song: None,
                jukebox_song_name: Some(reader.read_identifier()?),
                show_in_tooltip: reader.read_bool()?,
            })
        }
    }
}
// TODO: The wiki says that these are all Optional, but not when they are present.
Field!(JukeboxSong, {
    sound_event: SoundEvent,
    description: TextComponent,
    duration: Float,
    output: VarInt,
});
Field!(LodestoneTracker, {
    pub has_global_position: bool,
    pub dimension: Identifier,
    pub position: Position,
    pub tracked: bool,
});
Field!(Fireworks, {
    pub flight_duration: VarInt,
    pub explosions: PrefixedArray<FireworkExplosion>,
});
Field!(Profile, {
    pub name: PrefixedOptional<String>,
    pub uuid: PrefixedOptional<UUID>,
    pub property: PrefixedArray<ProfileProperty>
});
Field!(ProfileProperty, {
    pub name: String,
    pub value: String,
    pub signature: PrefixedOptional<String>,
});
#[derive(Clone, Debug)]
pub struct BannerPattern {
    pub pattern_type: VarInt,
    pub asset_id: Option<Identifier>,
    pub translation_key: Option<String>,
    pub color: DyeColor,
}
impl Field for BannerPattern {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_var_int(self.pattern_type);
        if self.pattern_type == 0 {
            res = res.into_iter().chain(encode_identifier(self.asset_id.clone().expect("pattern_type is 0, but asset_id is None")))
                .chain(encode_string(self.translation_key.clone().expect("pattern_type is 0, but translation_key is None")))
                .collect();
        }
        res.into_iter().chain(self.color.clone().to_bytes()).collect()
    }

    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let pattern_type = reader.read_var_int()?;
        if pattern_type == 0 {
            Ok(Self {
                pattern_type,
                asset_id: Some(reader.read_identifier()?),
                translation_key: Some(reader.read_string()?),
                color: reader.read()?
            })
        } else { 
            Ok(Self {
                pattern_type,
                asset_id: None,
                translation_key: None,
                color: reader.read()?
            })
        }
    }
}
Field!(BlockState, {
    pub name: String,
    pub value: String,
});
Field!(Bee, {
    pub entity_data: NBT,
    pub ticks_in_hive: VarInt,
    pub min_ticks_in_hive: VarInt,
});

VarIntEnum!(ConsumeEffect, {
    ApplyEffects: ApplyEffects
    RemoveEffects: IDSet,
    ClearAllEffects,
    TeleportRandomly: Float,
    PlaySound: SoundEvent
});
Field!(ApplyEffects, {
    effects: PrefixedArray<PotionEffect>, // TODO: verify that this is indeed a PrefixedArray. The wiki says that it's an Array
    probability: Float,
});
Field!(PotionEffect, {
    type_id: VarInt,
    details: PotionEffectDetail,
});
Field!(PotionEffectDetail, {
    amplifier: VarInt,
    duration: VarInt,
    ambient: bool,
    show_particles: bool,
    show_icon: bool,
    hidden_effect: PrefixedOptional<Box<PotionEffectDetail>>
});
Field!(SoundEvent, {
    sound_name: Identifier,
    fixed_range: PrefixedOptional<Float>
});
Field!(Instrument, {
    sound_event: IdOr<SoundEvent>,
    use_duration: Float,
    range: Float,
    description: TextComponent,
});
Field!(FireworkExplosion, {
    shape: FireworkExplosionShape,
    colors: PrefixedArray<Int>,
    fade_colors: PrefixedArray<Int>,
    has_trail: bool,
    has_twinkle: bool,
});
VarIntEnum!(FireworkExplosionShape, {
    SmallBall,
    LargeBall,
    Star,
    Creeper,
    Burst,
});
VarIntEnum!(DyeColor, {
    White,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
});