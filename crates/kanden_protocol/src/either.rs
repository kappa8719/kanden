use std::fmt::Debug;

use byteorder::{BigEndian, ReadBytesExt};

use crate::{Decode, Encode};

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Encode for Either<L, R>
where
    L: Encode,
    R: Encode,
{
    fn encode(&self, w: impl std::io::Write) -> anyhow::Result<()> {
        match self {
            Either::Left(left) => left.encode(w),
            Either::Right(right) => right.encode(w),
        }
    }
}

impl<'a, L, R> Decode<'a> for Either<L, R>
where
    L: Decode<'a>,
    R: Decode<'a>,
{
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        let is_left = r.read_u8()? == 1;
        if is_left {
            Ok(Self::Left(L::decode(r)?))
        } else {
            Ok(Self::Right(R::decode(r)?))
        }
    }
}

impl<L, R> Debug for Either<L, R>
where
    L: Debug,
    R: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left(arg0) => f.debug_tuple("Left").field(arg0).finish(),
            Self::Right(arg0) => f.debug_tuple("Right").field(arg0).finish(),
        }
    }
}

impl<L, R> PartialEq for Either<L, R>
where
    L: PartialEq,
    R: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Left(l0), Self::Left(r0)) => l0 == r0,
            (Self::Right(l0), Self::Right(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl<L, R> Eq for Either<L, R>
where
    L: Eq,
    R: Eq,
{
}

impl<L, R> Clone for Either<L, R>
where
    L: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Left(arg0) => Self::Left(arg0.clone()),
            Self::Right(arg0) => Self::Right(arg0.clone()),
        }
    }
}

impl<L, R> Copy for Either<L, R>
where
    L: Copy,
    R: Copy,
{
}
