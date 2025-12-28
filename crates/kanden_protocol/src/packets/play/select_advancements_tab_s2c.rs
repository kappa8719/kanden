use std::borrow::Cow;

use kanden_ident::Ident;

use crate::{Decode, Encode, Packet};

#[derive(Clone, Debug, Encode, Decode, Packet)]
pub struct SelectAdvancementsTabS2c<'a> {
    pub identifier: Option<Ident<Cow<'a, str>>>,
}
