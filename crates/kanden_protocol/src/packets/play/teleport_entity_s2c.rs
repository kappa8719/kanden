use kanden_math::DVec3;

use crate::{realtive::Relative, Decode, Encode, Packet, VarInt};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct TeleportEntityS2c {
    pub entity_id: VarInt,
    pub position: DVec3,
    pub delta_movement: DVec3,
    pub yaw: f32,
    pub pitch: f32,
    pub relative: Relative,
    pub on_ground: bool,
}
