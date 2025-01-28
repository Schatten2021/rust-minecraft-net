use std::fmt::Debug;
use minecraft_net_proc::{Field, Field_old};
use crate::fields::{encode_bool, encode_int, encode_long, encode_string, encode_var_int};
use crate::{Field, PacketReader, Result};
use crate::fields::types::{Identifier, Int, PrefixedArray, PrefixedOptional, VarInt, NBT};

pub mod status;
pub mod handshake;
pub mod login;
pub mod configuration;
pub mod play;

#[derive(Debug, Clone)]
pub struct Position {
    x: i32,
    y: i16,
    z: i32,
}
impl Position {
    pub fn new(x: i32, y: i16, z: i32) -> Position {
        Position { x, y, z }
    }
}
impl Field for Position {
    fn to_bytes(&self) -> Vec<u8> {
        let val = ((self.x as i64 & 0x3FFFFFF) << 38) | ((self.z as i64 & 0x3FFFFFF) << 12) | (self.y as i64 & 0xFFF);
        encode_long(val)
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Position> {
        let val = reader.read_long();
        let mut x = ((val >> 38) & 0x3FFFFFF) as i32;
        let mut z = ((val >> 12) & 0x3FFFFFF) as i32;
        let mut y = (val & 0xFFF) as i16;
        if x >= 1 << 25 { x -= 1 << 26 }
        if y >= 1 << 11 { y -= 1 << 12 }
        if z >= 1 << 25 { z -= 1 << 26 }
        Ok(Position {x, y, z})
    }
}
pub use slot::*;
mod slot {
    use minecraft_net_proc::{Field, VarIntEnum};
    use crate::fields::{encode_prefixed_array};
    use crate::fields::types::{Double, Float, Int, UUID};
    use super::*;
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

}
pub use block_predicate::*;
mod block_predicate {
    use crate::fields::encode_string;
    use super::*;
    #[derive(Debug, Clone, Field_old)]
    pub struct BlockPredicate {
        pub blocks: PrefixedOptional<IDSet>,
        pub properties: PrefixedOptional<PrefixedArray<Property>>,
        pub nbt: PrefixedOptional<NBT>,
    }
    #[derive(Debug, Clone)]
    pub struct IDSet {
        pub r#type: VarInt,
        pub tag_name: Option<String>,
        pub ids: Option<Vec<VarInt>>
    }
    impl Field for IDSet {
        fn to_bytes(&self) -> Vec<u8> {
            let mut res = encode_var_int(self.r#type);
            if self.r#type == 0 {
                res.append(&mut encode_string(self.tag_name.clone().expect("Tag name not provided")));
            } else {
                res.append(&mut self.ids.clone().expect("type not 0 but no ids given")
                    .iter().flat_map(|id| encode_var_int(*id)).collect());
            }
            res
        }
        fn from_reader(reader: &mut PacketReader) -> Result<Self> {
            let r#type = reader.read_var_int()?;
            if r#type == 0 {
                Ok(Self {
                    r#type,
                    tag_name: Some(reader.read_identifier()?),
                    ids: None
                })
            } else {
                let len = (r#type - 1) as usize;
                let mut vec = Vec::with_capacity(len);
                for _ in 0..len {
                    vec.push(reader.read_var_int()?);
                }
                Ok(Self {
                    r#type,
                    tag_name: None,
                    ids: Some(vec)
                })
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct Property {
        name: String,
        is_exact_match: bool,
        exact_value: Option<String>,
        min_value: Option<String>,
        max_value: Option<String>,
    }
    impl Field for Property {
        fn to_bytes(&self) -> Vec<u8> {
            let mut res = encode_string(self.name.clone());
            res.append(&mut encode_bool(self.is_exact_match));
            if self.is_exact_match {
                res.append(&mut encode_string(self.exact_value.clone().expect("Property value not provided")));
            } else {
                res.append(&mut encode_string(self.min_value.clone().expect("Property min value not provided")));
                res.append(&mut encode_string(self.max_value.clone().expect("Property max value not provided")));
            }
            res
        }
        fn from_reader(reader: &mut PacketReader) -> Result<Self> {
            let name = reader.read_string()?;
            let is_exact_match = reader.read_bool()?;
            if is_exact_match {
                Ok(Self {
                    name, is_exact_match,
                    exact_value: Some(reader.read_string()?),
                    min_value: None,
                    max_value: None,
                })
            } else {
                Ok(Self {
                    name, is_exact_match,
                    exact_value: None,
                    min_value: Some(reader.read_string()?),
                    max_value: Some(reader.read_string()?),
                })
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct IdOr<T: Field + Clone> {
    pub id: VarInt,
    pub value: Option<T>,
}
impl<T: Field + Clone> Field for IdOr<T> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_var_int(self.id);
        if self.id == 0 {
            res.append(&mut self.value.clone().expect("invalid IdOr field. Requires either ID to be non-zero or a value").to_bytes());
        }
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let id = reader.read_var_int()?;
        if id == 0 {
            Ok(Self {
                id,
                value: Some(T::from_reader(reader)?),
            })
        } else {
            Ok(Self {
                id, value: None,
            })
        }
    }
}
impl<T: Clone + Field> IdOr<T> {
    pub fn with_id(id: VarInt) -> Self {
        Self {id, value: None}
    }
    pub fn with_value(value: T) -> Self {
        Self {id: 0, value: Some(value) }
    }
}
#[derive(Clone, Debug)]
pub enum IDSet {
    TagName(Identifier),
    Ids(Vec<VarInt>),
}
impl Field for IDSet {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            IDSet::TagName(name) => vec![encode_var_int(0), encode_string(name.clone())].iter().flatten().cloned().collect::<Vec<u8>>(),
            IDSet::Ids(ids) => vec![
                encode_var_int(ids.len() as i32 + 1),
                ids.iter().flat_map(|id| encode_var_int(*id)).collect::<Vec<u8>>(),
            ].iter().flatten().cloned().collect::<Vec<u8>>(),
        }
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let t = reader.read_var_int()?;
        if t == 0 {
            Ok(Self::TagName(reader.read_identifier()?))
        } else {
            let mut res: Vec<VarInt> = Vec::with_capacity((t - 1) as usize);
            for _ in 0..(t-1) {
                res.push(reader.read_var_int()?);
            }
            Ok(Self::Ids(res))
        }
    }
}
Field!(BitSet, {
    data: PrefixedArray<Long>
});
impl BitSet {
    pub fn is_set(&self, i: usize) -> bool {
        self.data[i / 64] & (1 << (i % 64)) != 0
    }
    pub fn set(&mut self, i: usize) {
        self.extend_to(i);
        self.set_unchecked(i);
    }
    pub fn set_unchecked(&mut self, i: usize) {
        self.data[i / 64] |= 1 << (i % 64);
    }
    pub fn reset(&mut self, i: usize) {
        self.extend_to(i);
        self.reset_unchecked(i);
    }
    pub fn reset_unchecked(&mut self, i: usize) {
        self.data[i / 64] &= !(1 << (i % 64));
    }
    pub fn extend_to(&mut self, i: usize) {
        let to_add = (i / 8) + if i % 8 != 0 {1} else {0};
        self.data.resize(to_add, 0);
    }
}
mod chunk_and_light {
    use super::*;
    Field!(ChunkData, {
        height_maps: NBT,
        data: PrefixedArray<Byte>,
        block_entities: PrefixedArray<BlockEntity>,
    });
    Field!(BlockEntity, {
        packed_xz: UByte,
        y: Short,
        r#type: VarInt,
        data: NBT,
    });
    Field!(LightData, {
        sky_light_mask: BitSet,
        block_light_mask: BitSet,
        empty_sky_light_mask: BitSet,
        empty_block_light_mask: BitSet,
        sky_light_arrays: PrefixedArray<PrefixedArray<Byte>>,
        block_light_arrays: PrefixedArray<PrefixedArray<Byte>>,
    });
}
use chunk_and_light::*;
mod particles {
    use minecraft_net_proc::{Field, VarIntEnum};
    use crate::packets::{Position, Slot};

    VarIntEnum!(Particle, {
        AngryVillager,
        Block: VarInt,
        BlockMarker: VarInt,
        Bubble,
        Cloud,
        Crit,
        DamageIndicator,
        DragonBreath,
        DrippingLava,
        FallingLava,
        LandingLava,
        DrippingWater,
        FallingWater,
        Dust: Dust,
        DustColorTransition: DustColorTransition,
        Effect,
        ElderGuardian,
        EnchantedHit,
        Enchant,
        EndRod,
        EntityEffect: Int,
        ExplosionEmitter,
        Explosion,
        Gust,
        SmallGust,
        GustEmitterLarge,
        GustEmitterSmall,
        SonicBoom,
        FallingDust: VarInt,
        Firework,
        Fishing,
        Flame,
        Infested,
        CherryLeaves,
        PaleOakLeaves,
        SculkSoul,
        SculkCharge: Float,
        SculkChargePop,
        SoulFireFlame,
        Soul,
        Flash,
        HappyVillager,
        Composter,
        Heart,
        InstantEffect,
        Item: Slot,
        Vibration: Vibration,
        Trail: Trail,
        ItemSlime,
        ItemCobweb,
        ItemSnowball,
        LargeSmoke,
        Lava,
        Mycelium,
        Note,
        Poof,
        Portal,
        Rain,
        Smoke,
        WhiteSmoke,
        Sneeze,
        Spit,
        SquidInk,
        SweeepAttack,
        TotemOfUndying,
        Underwater,
        Splash,
        Witch,
        BubblePop,
        CurrentDown,
        BubbleColumnUp,
        Nautilus,
        Dolphin,
        CampfireCosySmoke,
        CampfireSignalSmoke,
        DrippingHoney,
        FallingHoney,
        LandingHoney,
        FallingNectar,
        FallingSporeBlossom,
        Ash,
        CrimsonSpore,
        WarpedSpore,
        SporeBlossomAir,
        DrippingObsidianTear,
        FallingObsidianTear,
        LandingObsidianTear,
        ReversePortal,
        WhiteAsh,
        SmallFlame,
        Snowflake,
        DrippingDripstoneLava,
        FallingDripstoneLava,
        DrippingDripstoneWater,
        FallingDripstoneWater,
        GlowSquidInk,
        Glow,
        WaxOn,
        WaxOff,
        ElectricSpark,
        Scrape,
        Shriek: VarInt,
        EggCrack,
        DustPlume,
        TrialSpawnerDetection,
        TrialSpawnerDetectionOminous,
        VaultConnection,
        DustPillar,
        OminousSpawning,
        RaidOmen,
        TrialOmen,
        BlockCrumble: VarInt,
    });
    Field!(Dust, {
        color: Int,
        scale: Float,
    });
    Field!(DustColorTransition, {
        from_color: Int,
        to_color: Int,
        scale: Float,
    });
    Field!(Vibration, {
        position_source_type: VarInt,
        block_position: Position,
        entity_id: VarInt,
        entity_eye_height: Float,
        ticks: VarInt,
    });
    Field!(Trail, {
        x: Double,
        y: Double,
        z: Double,
        color: Int,
        duration: VarInt,
    });
}
use particles::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct TeleportFlags(Int);
impl TeleportFlags {
    pub fn new(relative_x: bool,
               relative_y: bool,
               relative_z: bool,
               relative_yaw: bool,
               relative_pitch: bool,
               relative_velocity_x: bool,
               relative_velocity_y: bool,
               relative_velocity_z: bool,
               rotation_first: bool) -> Self {
        let mut new = Self::default();
        new.set_relative_x(relative_x);
        new.set_relative_y(relative_y);
        new.set_relative_z(relative_z);
        new.set_relative_yaw(relative_yaw);
        new.set_relative_pitch(relative_pitch);
        new.set_relative_velocity_x(relative_velocity_x);
        new.set_relative_velocity_y(relative_velocity_y);
        new.set_relative_velocity_z(relative_velocity_z);
        new.set_rotation_first(rotation_first);
        new
    }
    fn apply_mask(&mut self, index: usize, value: bool) {
        if value {
            self.0 |= 1 << index;
        } else {
            self.0 &= !(1 << index);
        }
    }
    pub fn set_relative_x(&mut self, new: bool) -> () {
        self.apply_mask(0, new)
    }
    pub fn set_relative_y(&mut self, new: bool) -> () {
        self.apply_mask(1, new)
    }
    pub fn set_relative_z(&mut self, new: bool) -> () {
        self.apply_mask(2, new)
    }
    pub fn set_relative_yaw(&mut self, new: bool) -> () {
        self.apply_mask(3, new)
    }
    pub fn set_relative_pitch(&mut self, new: bool) -> () {
        self.apply_mask(4, new)
    }
    pub fn set_relative_velocity_x(&mut self, new: bool) -> () {
        self.apply_mask(5, new)
    }
    pub fn set_relative_velocity_y(&mut self, new: bool) -> () {
        self.apply_mask(6, new)
    }
    pub fn set_relative_velocity_z(&mut self, new: bool) -> () {
        self.apply_mask(7, new)
    }
    pub fn set_rotation_first(&mut self, new: bool) -> () {
        self.apply_mask(8, new)
    }
}
impl Field for TeleportFlags {
    fn to_bytes(&self) -> Vec<u8> {
        encode_int(self.0)
    }

    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self(reader.read_int()))
    }
}
impl Default for TeleportFlags {
    fn default() -> Self {
        Self(0)
    }
}