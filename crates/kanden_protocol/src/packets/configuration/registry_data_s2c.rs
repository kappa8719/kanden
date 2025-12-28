use std::borrow::Cow;

use indexmap::IndexMap;
use kanden_ident::Ident;
use kanden_nbt::Compound;

use crate::{Decode, Encode, Packet, PacketState};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration)]
// After the server and the client have negotiated the required registry data,
// the server sends this packet for each registry to the client.
pub struct RegistryDataS2c<'a> {
    // The id of the registry
    pub id: Ident<Cow<'a, str>>,
    // The id of the entries and the entry data itself
    pub entries: IndexMap<Ident<Cow<'a, str>>, Option<Compound>>,
}
