use kanden_math::DVec3;

use crate::{Decode, Encode, Packet, VarInt};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct EntityPositionSyncS2c {
    pub entity_id: VarInt,
    pub position: DVec3,
    pub delta_movement: DVec3,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
