use crate::{Decode, Encode};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug, Encode, Decode)]
pub enum Arm {
    Left,
    #[default]
    Right,
}
