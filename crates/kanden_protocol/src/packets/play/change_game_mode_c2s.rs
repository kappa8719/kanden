use crate::{Decode, Encode, GameMode, Packet};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode, Packet)]
pub struct ChangeGameModeC2s {
    pub game_mode: GameMode,
}
