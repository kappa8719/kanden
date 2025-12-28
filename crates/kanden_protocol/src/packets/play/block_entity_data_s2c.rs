use std::borrow::Cow;

use kanden_generated::block::BlockEntityKind;
use kanden_nbt::Compound;

use crate::{BlockPos, Decode, Encode, Packet};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct BlockEntityDataS2c<'a> {
    pub location: BlockPos,
    pub kind: BlockEntityKind,
    pub data: Cow<'a, Compound>,
}
