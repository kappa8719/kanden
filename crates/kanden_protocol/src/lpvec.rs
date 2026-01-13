use std::ops::Deref;

use byteorder::{BigEndian, ReadBytesExt};
use kanden_math::Vec3;

use crate::{Decode, Encode, VarInt};

#[derive(Clone, Copy, PartialEq)]
pub struct LpVec3(pub Vec3);

impl Deref for LpVec3 {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Encode for LpVec3 {
    fn encode(&self, mut w: impl std::io::Write) -> anyhow::Result<()> {
        let src = self.0;
        let x = sanitize(src.x as f64);
        let y = sanitize(src.y as f64);
        let z = sanitize(src.z as f64);
        let max = f64::max(x.abs(), f64::max(y.abs(), z.abs()));
        if max < 3.051944088384301E-5 {
            w.write(&[0])?;
            return Ok(());
        }

        let max = max.ceil() as i64;
        let lower = (max & 3) != max;
        let masked = if lower { (max & 3) | 4 } else { max };
        let x = pack(x / max as f64) << 3;
        let y = pack(y / max as f64) << 18;
        let z = pack(z / max as f64) << 33;
        let combined = masked | x | y | z;

        let buf = [combined as i8, (combined >> 8) as i8];
        buf.encode(&mut w)?;
        ((combined >> 16) as i32).encode(&mut w)?;

        if lower {
            VarInt((max >> 2) as i32).encode(&mut w)?;
        }

        Ok(())
    }
}

impl<'a> Decode<'a> for LpVec3 {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        let a = r.read_u8()?;
        if a == 0 {
            return Ok(Self(Vec3::ZERO));
        }

        let b = r.read_u8()?;
        let c = r.read_u32::<BigEndian>()?;
        let destructed = ((c as i64) << 16) | ((b as i64) << 8) | a as i64;
        let mut lower = (c & 3) as u64;
        if a & 4 == 4 {
            let int = VarInt::decode(r)?.0 as u64;
            lower |= (int & 4294967295) << 2;
        }

        Ok(Self(
            Vec3 {
                x: unpack(destructed >> 3) as f32,
                y: unpack(destructed >> 18) as f32,
                z: unpack(destructed >> 33) as f32,
            } * lower as f32,
        ))
    }
}

fn sanitize(d: f64) -> f64 {
    if d.is_nan() {
        0.0
    } else {
        d.clamp(-1.7179869183E10, 1.7179869183E10)
    }
}

fn pack(d: f64) -> i64 {
    let value = (d * 0.5 + 0.5) * 32766.0;
    value.round() as i64
}

fn unpack(l: i64) -> f64 {
    f64::min((l & 32767) as f64, 32766.0) * 2.0 / 32766.0 - 1.0
}
