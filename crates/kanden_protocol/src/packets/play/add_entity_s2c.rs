use kanden_math::{DVec3, I8Vec3};
use uuid::Uuid;

use crate::{ByteAngle, Decode, Encode, Packet, VarInt, Velocity};

/// Sent by the server when a vehicle or other non-living entity is created.
///
/// wiki : [Spawn Entity](https://wiki.vg/Protocol#Spawn_Experience_Orb)
#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
pub struct AddEntityS2c {
    pub entity_id: VarInt,
    pub object_uuid: Uuid,
    pub kind: VarInt, // TODO: EntityKind in kanden_generated?
    pub position: DVec3,
    pub velocity: Velocity,
    pub pitch: ByteAngle,
    pub yaw: ByteAngle,
    pub head_yaw: ByteAngle,
    pub data: VarInt,
}
