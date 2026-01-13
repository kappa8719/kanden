use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// RGB color value represented as a 24-bit unsigned integer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RGB(pub u32);

impl RGB {
    /// Creates a new RGB color from a u32 value
    pub const fn new(value: u32) -> Self {
        Self(value & 0xFFFFFF)
    }

    /// Creates an RGB color from individual red, green, and blue components (0-255)
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    /// Returns the red component (0-255)
    pub const fn red(self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    /// Returns the green component (0-255)
    pub const fn green(self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    /// Returns the blue component (0-255)
    pub const fn blue(self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Returns the raw u32 value
    pub const fn value(self) -> u32 {
        self.0
    }

    /// Returns the color as a hex string with '#' prefix
    pub fn to_hex_string(self) -> String {
        format!("#{:06x}", self.0)
    }

    /// Parses a hex color string (with or without '#' prefix)
    pub fn from_hex_string(s: &str) -> Result<Self, std::num::ParseIntError> {
        let hex_str = s.strip_prefix('#').unwrap_or(s);
        u32::from_str_radix(hex_str, 16).map(Self::new)
    }
}

impl Serialize for RGB {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_hex_string())
    }
}

impl<'de> Deserialize<'de> for RGB {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_hex_string(&s).map_err(serde::de::Error::custom)
    }
}

/// ARGB color value represented as a 32-bit unsigned integer (alpha + RGB)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ARGB(pub u32);

impl ARGB {
    /// Creates a new ARGB color from a u32 value
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Creates an ARGB color from individual alpha, red, green, and blue components (0-255)
    pub const fn from_argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    /// Creates an ARGB color from RGB components with full opacity
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from_argb(255, r, g, b)
    }

    /// Returns the alpha component (0-255)
    pub const fn alpha(self) -> u8 {
        ((self.0 >> 24) & 0xFF) as u8
    }

    /// Returns the red component (0-255)
    pub const fn red(self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    /// Returns the green component (0-255)
    pub const fn green(self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    /// Returns the blue component (0-255)
    pub const fn blue(self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Returns the raw u32 value
    pub const fn value(self) -> u32 {
        self.0
    }

    /// Returns the color as a hex string with '#' prefix (includes alpha)
    pub fn to_hex_string(self) -> String {
        format!("#{:08x}", self.0)
    }

    /// Parses a hex color string (with or without '#' prefix)
    /// Supports both 6-digit (RGB, assumes full alpha) and 8-digit (ARGB) formats
    pub fn from_hex_string(s: &str) -> Result<Self, std::num::ParseIntError> {
        let hex_str = s.strip_prefix('#').unwrap_or(s);

        match hex_str.len() {
            // RGB
            6 => {
                let rgb = u32::from_str_radix(hex_str, 16)?;
                Ok(Self::new(0xFF000000 | rgb))
            }
            // ARGB
            _ => u32::from_str_radix(hex_str, 16).map(Self::new),
        }
    }

    /// Returns true if the color is fully opaque (alpha = 255)
    pub const fn is_opaque(self) -> bool {
        self.alpha() == 255
    }

    /// Returns true if the color is fully transparent (alpha = 0)
    pub const fn is_transparent(self) -> bool {
        self.alpha() == 0
    }
}

impl From<u32> for ARGB {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<ARGB> for u32 {
    fn from(color: ARGB) -> Self {
        color.0
    }
}

impl From<RGB> for ARGB {
    fn from(rgb: RGB) -> Self {
        Self::new(0xFF000000 | rgb.0)
    }
}

impl Serialize for ARGB {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_hex_string())
    }
}

impl<'de> Deserialize<'de> for ARGB {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_hex_string(&s).map_err(serde::de::Error::custom)
    }
}
