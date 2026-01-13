use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use kanden_entity::{
    entity::{self, DataSharedFlags},
    Pose,
};
pub use kanden_protocol::packets::play::player_command_c2s::PlayerCommand;
use kanden_protocol::packets::play::{PlayerCommandC2s, PlayerInputC2s};

use crate::{
    client::{Client, PlayerInputState},
    event_loop::{EventLoopPreUpdate, PacketEvent},
};

pub struct ClientCommandPlugin;

impl Plugin for ClientCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SprintEvent>()
            .add_event::<SneakEvent>()
            .add_event::<JumpWithHorseEvent>()
            .add_event::<LeaveBedEvent>()
            .add_systems(EventLoopPreUpdate, handle_client_command);
    }
}

#[derive(Event, Copy, Clone, PartialEq, Eq, Debug)]
pub struct SprintEvent {
    pub client: Entity,
    pub state: SprintState,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SprintState {
    Start,
    Stop,
}

#[derive(Event, Copy, Clone, PartialEq, Eq, Debug)]
pub struct SneakEvent {
    pub client: Entity,
    pub state: SneakState,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SneakState {
    Start,
    Stop,
}

#[derive(Event, Copy, Clone, PartialEq, Eq, Debug)]
pub struct JumpWithHorseEvent {
    pub client: Entity,
    pub state: JumpWithHorseState,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum JumpWithHorseState {
    Start {
        /// The power of the horse jump in `0..=100`.
        power: u8,
    },
    Stop,
}

#[derive(Event, Copy, Clone, PartialEq, Eq, Debug)]
pub struct LeaveBedEvent {
    pub client: Entity,
}

fn handle_client_command(
    mut packets: EventReader<PacketEvent>,
    mut clients: Query<(
        &mut entity::DataPose,
        &mut DataSharedFlags,
        &mut PlayerInputState,
    )>,
    mut sprinting_events: EventWriter<SprintEvent>,
    mut sneaking_events: EventWriter<SneakEvent>,
    mut jump_with_horse_events: EventWriter<JumpWithHorseEvent>,
    mut leave_bed_events: EventWriter<LeaveBedEvent>,
) {
    for packet in packets.read() {
        if let Some(pkt) = packet.decode::<PlayerInputC2s>() {
            if let Ok((mut pose, mut flags, mut input_state)) = clients.get_mut(packet.client) {
                if !flags.sneaking() && pkt.flags.sneak() {
                    sneaking_events.send(SneakEvent {
                        client: packet.client,
                        state: SneakState::Start,
                    });

                    pose.0 = Pose::Sneaking;
                    flags.set_sneaking(true);
                }

                if flags.sneaking() && !pkt.flags.sneak() {
                    sneaking_events.send(SneakEvent {
                        client: packet.client,
                        state: SneakState::Stop,
                    });

                    pose.0 = Pose::Standing;
                    flags.set_sneaking(false);
                }

                input_state.forward = pkt.flags.forward();
                input_state.back = pkt.flags.back();
                input_state.left = pkt.flags.left();
                input_state.right = pkt.flags.right();
                input_state.jump = pkt.flags.jump();
                input_state.sneak = pkt.flags.sneak();
                input_state.sprint = pkt.flags.sprint();
            }
        }
        if let Some(pkt) = packet.decode::<PlayerCommandC2s>() {
            match pkt.action {
                PlayerCommand::StopSleeping => {
                    leave_bed_events.send(LeaveBedEvent {
                        client: packet.client,
                    });
                }
                PlayerCommand::StartSprinting => {
                    if let Ok((_, mut flags, _)) = clients.get_mut(packet.client) {
                        flags.set_sprinting(true);
                    }

                    sprinting_events.send(SprintEvent {
                        client: packet.client,
                        state: SprintState::Start,
                    });
                }
                PlayerCommand::StopSprinting => {
                    if let Ok((_, mut flags, _)) = clients.get_mut(packet.client) {
                        flags.set_sprinting(false);
                    }

                    sprinting_events.send(SprintEvent {
                        client: packet.client,
                        state: SprintState::Stop,
                    });
                }
                PlayerCommand::StartRidingJump => {
                    jump_with_horse_events.send(JumpWithHorseEvent {
                        client: packet.client,
                        state: JumpWithHorseState::Start {
                            power: pkt.jump_boost.0 as u8,
                        },
                    });
                }
                PlayerCommand::StopRidingJump => {
                    jump_with_horse_events.send(JumpWithHorseEvent {
                        client: packet.client,
                        state: JumpWithHorseState::Stop,
                    });
                }
                PlayerCommand::OpenInventory => {} // TODO
                PlayerCommand::StartFallFlying => {
                    if let Ok((mut pose, _, _)) = clients.get_mut(packet.client) {
                        pose.0 = Pose::FallFlying;
                    }

                    // TODO.
                }
            }
        }
    }
}
