use std::borrow::Cow;

use kanden_ident::Ident;
use kanden_nbt::Compound;

use crate::{Decode, Encode, Packet, PacketState};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration)]
pub struct CustomClickActionC2s<'a> {
    pub id: Ident<Cow<'a, str>>,
    pub payload: Option<Compound>,
}
