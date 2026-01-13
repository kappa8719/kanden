use std::borrow::Cow;

use kanden_ident::Ident;

use crate::{Decode, Encode, Packet, PacketState};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration)]
pub struct CodeOfConductS2c<'a> {
    pub code_of_conduct: Cow<'a, str>,
}
