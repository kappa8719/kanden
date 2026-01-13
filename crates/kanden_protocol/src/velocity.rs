use std::fmt;

use derive_more::{From, Into};
use kanden_math::Vec3;

use crate::{lpvec::LpVec3, Decode, Encode};

/// Quantized entity velocity.
#[derive(Copy, Clone, PartialEq, Encode, Decode, From, Into)]
pub struct Velocity(pub LpVec3);

impl Velocity {
    // pub const MULTIPLIER: f32 = 8000.0 / 20.0;
    pub const MULTIPLIER: f32 = 1.0;

    pub fn from_array(array: [f32; 3]) -> Self {
        Self(LpVec3(Vec3::new(array[0], array[1], array[2])))
    }

    /// From meters/second.
    pub fn from_ms_f32(ms: [f32; 3]) -> Self {
        Self::from_array(ms.map(|v| Self::MULTIPLIER * v))
    }

    /// From meters/second.
    pub fn from_ms_f64(ms: [f64; 3]) -> Self {
        Self::from_array(ms.map(|v| ((Self::MULTIPLIER as f64) * v) as f32))
    }

    /// To meters/second.
    pub fn to_ms_f32(self) -> [f32; 3] {
        self.0.to_array().map(|v| v / (Self::MULTIPLIER))
    }

    /// To meters/second.
    pub fn to_ms_f64(self) -> [f64; 3] {
        self.0.to_array().map(|v| (v / (Self::MULTIPLIER)) as f64)
    }
}

impl fmt::Debug for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [x, y, z] = self.to_ms_f32();
        write!(f, "⟨{x},{y},{z}⟩ m/s")
    }
}

#[cfg(test)]
#[test]
fn velocity_from_ms() {
    let val_1 = Velocity::from_ms_f32([(); 3].map(|()| -3.3575)).0[0];
    let val_2 = Velocity::from_ms_f64([(); 3].map(|()| -3.3575)).0[0];

    assert_eq!(val_1, val_2);
    assert_eq!(val_1, -1343.0);
}
