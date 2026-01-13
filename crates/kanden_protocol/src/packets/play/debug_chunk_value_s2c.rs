use crate::{Decode, Encode, Packet};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode, Packet)]
pub struct DebugChunkValueS2c;
