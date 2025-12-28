#![cfg_attr(
    unstable_doc,
    doc = "**❗ NOTE:** This documentation is sourced from the `main` branch. If you're looking for the most recent stable release, go [here](https://docs.rs/kanden/latest/kanden/).\n\n---\n"
)]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/kanden-rs/kanden/main/assets/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/kanden-rs/kanden/main/assets/logo.svg"
)]
#![deny(
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    rustdoc::missing_crate_level_docs,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::bare_urls,
    rustdoc::invalid_html_tags
)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unused_lifetimes,
    unused_import_braces,
    unreachable_pub,
    clippy::dbg_macro
)]

use bevy_app::{PluginGroup, PluginGroupBuilder};

#[cfg(feature = "testing")]
pub mod testing;

#[cfg(test)]
mod tests;

#[cfg(feature = "log")]
pub use bevy_log as log;
use registry::biome::BiomePlugin;
use registry::dimension_type::DimensionTypePlugin;
#[cfg(feature = "advancement")]
pub use kanden_advancement as advancement;
#[cfg(feature = "anvil")]
pub use kanden_anvil as anvil;
#[cfg(feature = "boss_bar")]
pub use kanden_boss_bar as boss_bar;
#[cfg(feature = "command")]
pub use kanden_command as command;
#[cfg(feature = "command")]
pub use kanden_command_macros as command_macros;
#[cfg(feature = "inventory")]
pub use kanden_inventory as inventory;
pub use kanden_lang as lang;
#[cfg(feature = "network")]
pub use kanden_network as network;
#[cfg(feature = "player_list")]
pub use kanden_player_list as player_list;
use kanden_registry::RegistryPlugin;
#[cfg(feature = "scoreboard")]
pub use kanden_scoreboard as scoreboard;
use kanden_server::abilities::AbilitiesPlugin;
use kanden_server::action::ActionPlugin;
use kanden_server::client::ClientPlugin;
use kanden_server::client_command::ClientCommandPlugin;
use kanden_server::client_settings::ClientSettingsPlugin;
use kanden_server::custom_payload::CustomPayloadPlugin;
use kanden_server::entity::hitbox::HitboxPlugin;
use kanden_server::entity::EntityPlugin;
use kanden_server::event_loop::EventLoopPlugin;
use kanden_server::hand_swing::HandSwingPlugin;
use kanden_server::interact_block::InteractBlockPlugin;
use kanden_server::interact_entity::InteractEntityPlugin;
use kanden_server::interact_item::InteractItemPlugin;
use kanden_server::keepalive::KeepalivePlugin;
use kanden_server::layer::LayerPlugin;
use kanden_server::message::MessagePlugin;
use kanden_server::movement::MovementPlugin;
use kanden_server::op_level::OpLevelPlugin;
pub use kanden_server::protocol::status_effects;
use kanden_server::resource_pack::ResourcePackPlugin;
use kanden_server::status::StatusPlugin;
use kanden_server::status_effect::StatusEffectPlugin;
use kanden_server::teleport::TeleportPlugin;
pub use kanden_server::*;
#[cfg(feature = "weather")]
pub use kanden_weather as weather;
#[cfg(feature = "world_border")]
pub use kanden_world_border as world_border;

/// Contains the most frequently used items in Kanden projects.
///
/// This is usually glob imported like so:
///
/// ```no_run
/// use kanden::prelude::*; // Glob import.
///
/// let mut app = App::empty();
/// app.add_systems(Update, || println!("yippee!"));
/// app.update()
/// // ...
/// ```
pub mod prelude {
    pub use bevy_app::prelude::*;
    pub use bevy_ecs; // Needed for bevy_ecs macros to function correctly.
    pub use bevy_ecs::prelude::*;
    pub use uuid::Uuid;
    #[cfg(feature = "advancement")]
    pub use kanden_advancement::{
        event::AdvancementTabChangeEvent, Advancement, AdvancementBundle, AdvancementClientUpdate,
        AdvancementCriteria, AdvancementDisplay, AdvancementFrameType, AdvancementRequirements,
    };
    #[cfg(feature = "inventory")]
    pub use kanden_inventory::{
        CursorItem, Inventory, InventoryKind, InventoryWindow, InventoryWindowMut, OpenInventory,
    };
    #[cfg(feature = "network")]
    pub use kanden_network::{
        ConnectionMode, ErasedNetworkCallbacks, NetworkCallbacks, NetworkSettings, NewClientInfo,
        SharedNetworkState,
    };
    #[cfg(feature = "player_list")]
    pub use kanden_player_list::{PlayerList, PlayerListEntry};
    pub use kanden_registry::biome::{Biome, BiomeId, BiomeRegistry};
    pub use kanden_registry::dimension_type::{DimensionType, DimensionTypeRegistry};
    pub use kanden_server::action::{DiggingEvent, DiggingState};
    pub use kanden_server::block::{BlockKind, BlockState, PropName, PropValue};
    pub use kanden_server::client::{
        despawn_disconnected_clients, Client, Ip, OldView, OldViewDistance, Properties, Username,
        View, ViewDistance, VisibleChunkLayer, VisibleEntityLayers,
    };
    pub use kanden_server::client_command::{
        JumpWithHorseEvent, JumpWithHorseState, LeaveBedEvent, PlayerCommand, SneakEvent,
        SneakState, SprintEvent, SprintState,
    };
    pub use kanden_server::entity::hitbox::{Hitbox, HitboxShape};
    pub use kanden_server::entity::{
        EntityAnimation, EntityKind, EntityLayerId, EntityManager, EntityStatus, HeadYaw, Look,
        OldEntityLayerId, OldPosition, Position,
    };
    pub use kanden_server::event_loop::{
        EventLoopPostUpdate, EventLoopPreUpdate, EventLoopUpdate,
    };
    pub use kanden_server::ident::Ident;
    pub use kanden_server::interact_entity::{EntityInteraction, InteractEntityEvent};
    pub use kanden_server::layer::chunk::{
        Block, BlockRef, Chunk, ChunkLayer, LoadedChunk, UnloadedChunk,
    };
    pub use kanden_server::layer::{EntityLayer, LayerBundle};
    pub use kanden_server::math::{DVec2, DVec3, Vec2, Vec3};
    pub use kanden_server::message::SendMessage as _;
    pub use kanden_server::nbt::Compound;
    pub use kanden_server::protocol::packets::play::level_particles_s2c::Particle;
    pub use kanden_server::protocol::text::{Color, IntoText, Text};
    pub use kanden_server::spawn::{ClientSpawnQuery, ClientSpawnQueryReadOnly, RespawnPosition};
    pub use kanden_server::title::SetTitle as _;
    pub use kanden_server::{
        ident, BlockPos, ChunkPos, ChunkView, Despawned, Direction, GameMode, Hand, ItemKind,
        ItemStack, Server, UniqueId,
    };

    pub use super::DefaultPlugins;
}

/// This plugin group will add all the default plugins for a Kanden
/// application.
///
/// [`DefaultPlugins`] obeys Cargo feature flags. Users may exert control over
/// this plugin group by disabling `default-features` in their `Cargo.toml` and
/// enabling only those features that they wish to use.
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        #[allow(unused_mut)]
        let mut group = PluginGroupBuilder::start::<Self>()
            .add(ServerPlugin)
            .add(RegistryPlugin)
            .add(BiomePlugin)
            .add(DimensionTypePlugin)
            .add(EntityPlugin)
            .add(HitboxPlugin)
            .add(LayerPlugin)
            .add(ClientPlugin)
            .add(EventLoopPlugin)
            .add(MovementPlugin)
            .add(ClientCommandPlugin)
            .add(KeepalivePlugin)
            .add(InteractEntityPlugin)
            .add(ClientSettingsPlugin)
            .add(ActionPlugin)
            .add(TeleportPlugin)
            .add(MessagePlugin)
            .add(CustomPayloadPlugin)
            .add(HandSwingPlugin)
            .add(InteractBlockPlugin)
            .add(InteractItemPlugin)
            .add(OpLevelPlugin)
            .add(ResourcePackPlugin)
            .add(StatusPlugin)
            .add(StatusEffectPlugin)
            .add(AbilitiesPlugin);

        #[cfg(feature = "log")]
        {
            group = group.add(bevy_log::LogPlugin::default())
        }

        #[cfg(feature = "network")]
        {
            group = group.add(kanden_network::NetworkPlugin)
        }

        #[cfg(feature = "player_list")]
        {
            group = group.add(kanden_player_list::PlayerListPlugin)
        }

        #[cfg(feature = "inventory")]
        {
            group = group.add(kanden_inventory::InventoryPlugin)
        }

        #[cfg(feature = "anvil")]
        {
            group = group.add(kanden_anvil::AnvilPlugin)
        }

        #[cfg(feature = "advancement")]
        {
            group = group.add(kanden_advancement::AdvancementPlugin)
        }

        #[cfg(feature = "weather")]
        {
            group = group.add(kanden_weather::WeatherPlugin)
        }

        #[cfg(feature = "world_border")]
        {
            group = group.add(kanden_world_border::WorldBorderPlugin)
        }

        #[cfg(feature = "boss_bar")]
        {
            group = group.add(kanden_boss_bar::BossBarPlugin)
        }

        #[cfg(feature = "command")]
        {
            group = group.add(kanden_command::manager::CommandPlugin)
        }

        #[cfg(feature = "scoreboard")]
        {
            group = group.add(kanden_scoreboard::ScoreboardPlugin)
        }

        group
    }
}
