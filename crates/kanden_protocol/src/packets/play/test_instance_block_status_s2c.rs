use kanden_math::DVec3;
use kanden_text::Text;

use crate::{Decode, Encode, Packet};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct TestInstanceBlockStatusS2c {
    pub status: Text,
    pub size: Option<DVec3>,
}
