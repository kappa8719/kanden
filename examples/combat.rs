#![allow(clippy::type_complexity)]

use bevy_ecs::query::QueryData;
use kanden::entity::living::DataHealth;
use kanden::entity::{EntityId, EntityStatuses, OnGround, Velocity};
use kanden::math::Vec3Swizzles;
use kanden::protocol::lpvec::LpVec3;
use kanden::protocol::packets::play::{DamageEventS2c, HurtAnimationS2c};
use kanden::protocol::{Decode, Encode, VarInt, WritePacket};
use kanden::{prelude::*, Layer};
use rand::Rng;

const SPAWN_Y: i32 = 64;
const ARENA_RADIUS: i32 = 32;

/// Attached to every client.
#[derive(Component, Default)]
struct CombatState {
    /// The tick the client was last attacked.
    last_attacked_tick: i64,
    has_bonus_knockback: bool,
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(EventLoopUpdate, handle_combat_events)
        .add_systems(
            Update,
            (
                init_clients,
                despawn_disconnected_clients,
                teleport_oob_clients,
                debug,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
) {
    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    for z in -5..5 {
        for x in -5..5 {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    let mut rng = rand::thread_rng();

    // Create circular arena.
    for z in -ARENA_RADIUS..ARENA_RADIUS {
        for x in -ARENA_RADIUS..ARENA_RADIUS {
            let dist = f64::hypot(f64::from(x), f64::from(z)) / f64::from(ARENA_RADIUS);

            if dist > 1.0 {
                continue;
            }

            let block = if rng.gen::<f64>() < dist {
                BlockState::STONE
            } else {
                BlockState::DEEPSLATE
            };

            for y in 0..SPAWN_Y {
                layer.chunk.set_block([x, y, z], block);
            }
        }
    }

    commands.spawn(layer);
}

fn init_clients(
    mut clients: Query<
        (
            Entity,
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
    mut commands: Commands,
) {
    for (
        entity,
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
    ) in &mut clients
    {
        let layer = layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set([0.0, f64::from(SPAWN_Y) + 1.0, 0.0]);
        *game_mode = GameMode::Creative;

        commands.entity(entity).insert(CombatState::default());
    }
}

#[derive(QueryData)]
#[query_data(mutable)]
struct CombatQuery {
    id: &'static EntityId,
    client: &'static mut Client,
    velocity: &'static mut Velocity,
    look: &'static Look,
    pos: &'static Position,
    on_ground: &'static mut OnGround,
    state: &'static mut CombatState,
    statuses: &'static mut EntityStatuses,
    health: &'static mut DataHealth,
    layer: &'static mut VisibleChunkLayer,
}

fn handle_combat_events(
    server: Res<Server>,
    mut layers: Query<&mut ChunkLayer>,
    mut clients: Query<CombatQuery>,
    mut sprinting: EventReader<SprintEvent>,
    mut interact_entity: EventReader<InteractEntityEvent>,
) {
    for &SprintEvent { client, state } in sprinting.read() {
        if let Ok(mut client) = clients.get_mut(client) {
            client.state.has_bonus_knockback = state == SprintState::Start;
        }
    }

    for &InteractEntityEvent {
        client: attacker_client,
        entity: victim_client,
        ..
    } in interact_entity.read()
    {
        let Ok([mut attacker, mut victim]) = clients.get_many_mut([attacker_client, victim_client])
        else {
            // Victim or attacker does not exist, or the attacker is attacking itself.
            continue;
        };

        if server.current_tick() - victim.state.last_attacked_tick < 10 {
            // Victim is still on attack cooldown.
            continue;
        }

        victim.state.last_attacked_tick = server.current_tick();

        let victim_pos = victim.pos.0.xz();
        let attacker_pos = attacker.pos.0.xz();
        let dir = (attacker_pos - victim_pos).normalize().as_vec2();

        victim
            .velocity
            .apply_knockback(0.5, dir.x, dir.y, victim.on_ground.0);

        attacker.state.has_bonus_knockback = false;

        let Ok(mut layer) = layers.get_mut(victim.layer.0) else {
            return;
        };

        let mut layer_writer = layer.view_writer(victim.pos.0);
        layer_writer.write_packet(&DamageEventS2c {
            entity_id: VarInt(victim.id.get()),
            source_type_id: VarInt(29),
            source_cause_id: VarInt(-1),
            source_direct_id: VarInt(-1),
            source_pos: Some(attacker.pos.0.into()),
        });
        layer_writer.write_packet(&HurtAnimationS2c {
            entity_id: VarInt(victim.id.get()),
            yaw: victim.look.yaw,
        });
    }
}

fn teleport_oob_clients(mut clients: Query<&mut Position, With<Client>>) {
    for mut pos in &mut clients {
        if pos.0.y < 0.0 {
            pos.set([0.0, f64::from(SPAWN_Y), 0.0]);
        }
    }
}

fn debug(mut clients: Query<(&mut Client, &Velocity)>) {
    for (mut client, velocity) in clients.iter_mut() {
        let mut buf = Vec::new();
        velocity.0.encode(&mut buf).unwrap();
        client.send_action_bar_message(format!("{buf:?}"));
    }
}
