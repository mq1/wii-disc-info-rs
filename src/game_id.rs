// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use crate::{RegionCode, util::ArrayString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct GameID(ArrayString<6>);

impl Default for GameID {
    fn default() -> Self {
        Self(ArrayString::zero_filled())
    }
}

impl GameID {
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[must_use]
    #[inline]
    pub fn as_partial_str(&self) -> &str {
        self.0.as_str_until::<3>()
    }

    #[must_use]
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 6] {
        self.0.as_bytes()
    }

    #[must_use]
    #[inline]
    pub fn to_bytes(&self) -> [u8; 6] {
        self.0.to_bytes()
    }

    #[must_use]
    #[inline]
    pub fn system_code(&self) -> u8 {
        self.0.byte_at::<0>()
    }

    #[must_use]
    #[inline]
    pub fn region(&self) -> RegionCode {
        // Ratatouille (RLWW78) has a region byte of 'W', but it's actually a Scandinavian release
        if self.0 == *b"RLWW78" {
            return RegionCode::Scandinavia;
        }

        RegionCode::from_region_byte(self.0.byte_at::<3>())
    }

    #[must_use]
    #[inline]
    /// Compact representation of the game ID as a 32-bit unsigned integer
    pub fn to_u32(&self) -> u32 {
        // already validated in "validate characters"
        u32::from_str_radix(self.0.as_str(), 36).unwrap()
    }
}

impl TryFrom<ArrayString<6>> for GameID {
    type Error = std::io::Error;

    fn try_from(value: ArrayString<6>) -> Result<Self, Self::Error> {
        // validate length
        let len = value.len();
        if len != 4 && len != 6 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid game id length",
            ));
        }

        // validate characters
        if !value
            .as_bytes()
            .iter()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid game id",
            ));
        }

        Ok(Self(value))
    }
}

impl TryFrom<&[u8]> for GameID {
    type Error = std::io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        ArrayString::try_from(value)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            .and_then(Self::try_from)
    }
}

impl TryFrom<[u8; 6]> for GameID {
    type Error = std::io::Error;

    fn try_from(value: [u8; 6]) -> Result<Self, Self::Error> {
        ArrayString::try_from(value)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            .and_then(Self::try_from)
    }
}

impl std::str::FromStr for GameID {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<ArrayString<6>>()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            .and_then(Self::try_from)
    }
}

impl std::fmt::Display for GameID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl AsRef<str> for GameID {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
