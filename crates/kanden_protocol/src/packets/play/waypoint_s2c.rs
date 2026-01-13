use std::{borrow::Cow, io::Read};

use anyhow::{Context, Ok};
use byteorder::{BigEndian, ByteOrder};
use kanden_ident::Ident;
use uuid::Uuid;

use crate::{Decode, Either, Encode, Packet};

#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, Packet)]
pub struct WaypointS2c<'a> {
    pub operation: WaypointOperation,
    pub waypoint: Waypoint<'a>,
}

#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode)]
pub enum WaypointOperation {
    Track,
    Untrack,
    Update,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaypointIcon<'a> {
    style: Ident<Cow<'a, str>>,
    color: u32,
}

impl<'a> Decode<'a> for WaypointIcon<'a> {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        let mut buf = [0; 3];
        r.read_exact(&mut buf);
        let color = BigEndian::read_u24(&buf);
        Ok(Self {
            style: Decode::decode(r).context("failed to decode field `style` in `WaypointIcon`")?,
            color,
        })
    }
}

impl<'a> Encode for WaypointIcon<'a> {
    fn encode(&self, mut w: impl std::io::Write) -> anyhow::Result<()> {
        self.style
            .encode(&mut w)
            .context("failed to encode field `style` in `WaypointIcon`")?;

        let mut buf = [0; 3];
        BigEndian::write_u24(&mut buf, self.color);
        w.write_all(&buf);

        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode)]
/// Maps to TrackedWaypoint.Type
pub enum WaypointType {
    /// Maps to TrackedWaypoint.Type.Empty
    Empty,
    /// Maps to TrackedWaypoint.Type.VEC3I
    Vec3,
    /// Maps to TrackedWaypoint.Type.CHUNK
    Chunk,
    /// Maps to TrackedWaypoint.Type.AZIMUTH
    Azimuth,
}

#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode)]
pub struct Waypoint<'a> {
    pub identifier: Either<Uuid, Cow<'a, str>>,
    pub icon: WaypointIcon<'a>,
    pub ty: WaypointType,
}
