use std::{collections::HashMap, ops::Deref};

use kanden_entity::activity::Activity;
use kanden_ident::Ident;
use kanden_nbt::{compound, serde::ser::CompoundSerializer, Compound};
use kanden_protocol::Text;
use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};

use crate::serde::{ARGB, RGB};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MoonPhaseKeyframe {
    pub ticks: u32,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ParticleOptions {
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AmbientParticle {
    pub particle: ParticleOptions,
    pub probability: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Music {
    pub sound: Ident<String>,
    pub min_delay: i32,
    pub max_delay: i32,
    #[serde(default)]
    pub replace_current_music: BooleanIntRepr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct BackgroundMusic {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Music>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creative: Option<Music>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underwater: Option<Music>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AmbientMoodSettings {
    pub sound: Ident<String>,
    pub tick_delay: i32,
    pub block_search_extent: i32,
    pub offset: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AmbientAdditionsSettings {
    pub sound: Ident<String>,
    pub tick_chance: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AmbientSounds {
    /// Original path: *AmbientSounds::loop*
    #[serde(rename = "loop", skip_serializing_if = "Option::is_none")]
    pub looping: Option<Ident<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mood: Option<AmbientMoodSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additions: Option<AmbientAdditionsSettings>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BedRule {
    Always,
    WhenDark,
    Never,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BedRules {
    pub can_sleep: BedRule,
    pub can_set_spawn: BedRule,
    pub explodes: BooleanIntRepr,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<Text>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TriState {
    True,
    False,
    Default,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct BooleanIntRepr(pub bool);

impl Serialize for BooleanIntRepr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let int = if self.0 { 1 } else { 0 };
        serializer.serialize_i32(int)
    }
}

impl<'de> Deserialize<'de> for BooleanIntRepr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let int = i32::deserialize(deserializer)?;
        Ok(Self(int == 1))
    }
}

impl Deref for BooleanIntRepr {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum EnvironmentAttribute {
    #[serde(rename = "minecraft:visual/fog_color")]
    FogColor(RGB),
    #[serde(rename = "minecraft:visual/fog_start_distance")]
    FogStartDistance(f32),
    #[serde(rename = "minecraft:visual/fog_end_distance")]
    FogEndDistance(f32),
    #[serde(rename = "minecraft:visual/sky_fog_end_distance")]
    SkyFogEndDistance(f32),
    #[serde(rename = "minecraft:visual/cloud_fog_end_distance")]
    CloudFogEndDistance(f32),
    #[serde(rename = "minecraft:visual/water_fog_color")]
    WaterFogColor(RGB),
    #[serde(rename = "minecraft:visual/water_fog_start_distance")]
    WaterFogStartDistance(f32),
    #[serde(rename = "minecraft:visual/water_fog_end_distance")]
    WaterFogEndDistance(f32),
    #[serde(rename = "minecraft:visual/sky_color")]
    SkyColor(RGB),
    #[serde(rename = "minecraft:visual/sunrise_sunset_color")]
    SunriseSunsetColor(ARGB),
    #[serde(rename = "minecraft:visual/cloud_color")]
    CloudColor(ARGB),
    #[serde(rename = "minecraft:visual/cloud_height")]
    CloudHeight(f32),
    #[serde(rename = "minecraft:visual/sun_angle")]
    SunAngle(f32),
    #[serde(rename = "minecraft:visual/moon_angle")]
    MoonAngle(f32),
    #[serde(rename = "minecraft:visual/star_angle")]
    StarAngle(f32),
    #[serde(rename = "minecraft:visual/moon_phase")]
    MoonPhase { keyframes: Vec<MoonPhaseKeyframe> },
    #[serde(rename = "minecraft:visual/star_brightness")]
    StarBrightness(f32),
    #[serde(rename = "minecraft:visual/sky_light_color")]
    SkyLightColor(RGB),
    #[serde(rename = "minecraft:visual/sky_light_factor")]
    SkyLightFactor(f32),
    #[serde(rename = "minecraft:visual/default_dripstone_particle")]
    DefaultDripstoneParticle(ParticleOptions),
    #[serde(rename = "minecraft:visual/ambient_particles")]
    AmbientParticles(Vec<AmbientParticle>),
    #[serde(rename = "minecraft:audio/background_music")]
    BackgroundMusic(BackgroundMusic),
    #[serde(rename = "minecraft:audio/music_volume")]
    MusicVolume(f32),
    #[serde(rename = "minecraft:audio/ambient_sounds")]
    AmbientSounds(AmbientSounds),
    #[serde(rename = "minecraft:audio/firefly_bush_sounds")]
    FireflyBushSounds(BooleanIntRepr),
    #[serde(rename = "minecraft:gameplay/sky_light_level")]
    SkyLightLevel(f32),
    #[serde(rename = "minecraft:gameplay/can_start_raid")]
    CanStartRaid(BooleanIntRepr),
    #[serde(rename = "minecraft:gameplay/water_evaporates")]
    WaterEvaporates(BooleanIntRepr),
    #[serde(rename = "minecraft:gameplay/bed_rule")]
    BedRule(BedRules),
    #[serde(rename = "minecraft:gameplay/respawn_anchor_works")]
    RespawnAnchorWorks(BooleanIntRepr),
    #[serde(rename = "minecraft:gameplay/nether_portal_spawns_piglin")]
    NetherPortalSpawnsPiglins(BooleanIntRepr),
    #[serde(rename = "minecraft:gameplay/fast_lava")]
    FastLava(BooleanIntRepr),
    #[serde(rename = "minecraft:gameplay/increased_fire_burnout")]
    IncreasedFireBurnout(BooleanIntRepr),
    #[serde(rename = "minecraft:gameplay/eyeblossom_open")]
    EyeblossomOpen(TriState),
    #[serde(rename = "minecraft:gameplay/turtle_egg_hatch_chance")]
    TurtleEggHatchChance(f32),
    #[serde(rename = "minecraft:gameplay/piglins_zombify")]
    PiglinsZombify(BooleanIntRepr),
    #[serde(rename = "minecraft:gameplay/snow_golem_melts")]
    SnowGolemMelts(BooleanIntRepr),
    #[serde(rename = "minecraft:gameplay/creaking_active")]
    CreakingActive(BooleanIntRepr),
    #[serde(rename = "minecraft:gameplay/surface_slime_spawn_chance")]
    SurfaceSlimeSpawnChance(f32),
    #[serde(rename = "minecraft:gameplay/cat_waking_up_gift_chance")]
    CatWakingUpGiftChance(f32),
    #[serde(rename = "minecraft:gameplay/bees_stay_in_hive")]
    BeesStayInHive(BooleanIntRepr),
    #[serde(rename = "minecraft:gameplay/monsters_burn")]
    MonstersBurn(BooleanIntRepr),
    #[serde(rename = "minecraft:gameplay/can_pillager_patrol_spawn")]
    CanPillagerPatrolSpawn(BooleanIntRepr),
    #[serde(rename = "minecraft:gameplay/villager_activity")]
    VillagerActivity(Activity),
    #[serde(rename = "minecraft:gameplay/baby_villager_activity")]
    BabyVillagerActivity(Activity),
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct EnvironmentAttributeMap(Vec<EnvironmentAttribute>);

impl<'de> Deserialize<'de> for EnvironmentAttributeMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let compound: Compound = Compound::deserialize(deserializer)?;
        let mut attributes: Vec<EnvironmentAttribute> = Vec::new();

        for (key, value) in compound {
            let compound = compound! {
                "type" => key,
                "data" => value,
            };
            let attribute =
                EnvironmentAttribute::deserialize(compound).map_err(serde::de::Error::custom)?;
            attributes.push(attribute);
        }

        Ok(Self(attributes))
    }
}

impl Serialize for EnvironmentAttributeMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for attribute in self.0.iter() {
            let serialized = attribute
                .serialize(CompoundSerializer)
                .map_err(serde::ser::Error::custom)?;
            let key = serialized.get("type").unwrap();
            let value = serialized.get("data").unwrap();
            map.serialize_entry(key, value)?;
        }

        map.end()
    }
}
