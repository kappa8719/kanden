use std::borrow::Cow;

use kanden_ident::Ident;

use crate::{Decode, Encode, Packet, PacketState};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration)]
/// Clears dialog on client
pub struct ClearDialogS2c {
    // TODO
}
