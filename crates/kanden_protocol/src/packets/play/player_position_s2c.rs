use kanden_math::DVec3;

use crate::{realtive::Relative, Decode, Encode, Packet, VarInt};

#[derive(Copy, Clone, PartialEq, Debug, Encode, Decode, Packet)]
pub struct PlayerPositionS2c {
    pub teleport_id: VarInt,
    pub position: DVec3,
    pub velocity: DVec3,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: Relative,
}
