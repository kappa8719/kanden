use crate::{BlockPos, Decode, Encode, Packet};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode, Packet)]
pub struct GameTestHighlightPosS2c {
    pub absolute_pos: BlockPos,
    pub relative_pos: BlockPos,
}
