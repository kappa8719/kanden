use std::borrow::Cow;

use kanden_ident::Ident;

use crate::{Decode, Encode, Packet};

#[derive(Clone, Debug, Encode, Decode, Packet)]
/// Response to a cookie request from the server.
pub struct CookieResponseC2s<'a> {
    pub key: Ident<Cow<'a, str>>,
    pub payload: Option<Cow<'a, [u8]>>,
}
