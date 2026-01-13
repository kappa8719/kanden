use std::borrow::Cow;

use crate::{Decode, Encode, GlobalPos, Packet};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct SetDefaultSpawnPositionS2c<'a> {
    pub position: GlobalPos<Cow<'a, str>>,
    pub yaw: f32,
    pub pitch: f32,
}
