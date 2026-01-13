use bitfield_struct::bitfield;

use crate::{Decode, Encode};

#[bitfield(u32)]
#[derive(PartialEq, Eq, Encode, Decode)]
pub struct Relative {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub y_rot: bool,
    pub x_rot: bool,
    pub x_vel: bool,
    pub y_vel: bool,
    pub z_vel: bool,
    pub rot_vel: bool,
    #[bits(23)]
    _pad: u32,
}
