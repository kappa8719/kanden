use crate::{Decode, Encode, Packet, VarInt};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode, Packet)]
pub struct MountScreenOpenS2c {
    pub container_id: VarInt,
    pub inventory_columns: VarInt,
    pub entity_id: i32,
}
