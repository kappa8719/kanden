use std::borrow::Cow;

use kanden_ident::Ident;

use crate::block_pos::BlockPos;
use crate::{Decode, Encode};

#[derive(Clone, PartialEq, Eq, Debug, Encode)]
pub struct GlobalPos<S: AsRef<str>> {
    pub dimension_name: Ident<S>,
    pub position: BlockPos,
}

impl<'a> Decode<'a> for GlobalPos<Cow<'a, str>> {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        let dimension_name = Ident::<Cow<'a, str>>::decode(r)?;
        let position = BlockPos::decode(r)?;

        Ok(Self {
            dimension_name,
            position,
        })
    }
}

impl<S> Default for GlobalPos<S>
where
    S: AsRef<str>,
    Ident<S>: From<Ident<Cow<'static, str>>>,
{
    fn default() -> Self {
        Self {
            dimension_name: Ident::new("overworld").unwrap().into(),
            position: BlockPos::default(),
        }
    }
}
