// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use crate::RegionCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct GameID {
    inner: [u8; 6],
}

impl GameID {
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        let len = if self.inner[4] == 0 { 4 } else { 6 };

        // SAFETY: already validated
        #[allow(unsafe_code)]
        unsafe {
            std::str::from_utf8_unchecked(self.inner.get_unchecked(..len))
        }
    }

    #[must_use]
    #[inline]
    pub fn as_partial_str(&self) -> &str {
        // SAFETY: already validated
        #[allow(unsafe_code)]
        unsafe {
            std::str::from_utf8_unchecked(&self.inner[..3])
        }
    }

    #[must_use]
    #[inline]
    pub const fn as_bytes(&self) -> &[u8; 6] {
        &self.inner
    }

    #[must_use]
    #[inline]
    pub const fn to_bytes(&self) -> [u8; 6] {
        self.inner
    }

    #[must_use]
    #[inline]
    pub const fn system_code(&self) -> u8 {
        self.inner[0]
    }

    #[must_use]
    #[inline]
    pub const fn region(&self) -> RegionCode {
        // Ratatouille (RLWW78) has a region byte of 'W', but it's actually a Scandinavian release
        if matches!(&self.inner, b"RLWW78") {
            return RegionCode::Scandinavia;
        }

        RegionCode::from_region_byte(self.inner[3])
    }

    #[must_use]
    #[inline]
    /// Compact representation of the game ID as a 32-bit unsigned integer
    pub fn to_u32(&self) -> u32 {
        // SAFETY: already validated
        u32::from_str_radix(self.as_str(), 36).unwrap()
    }
}

impl TryFrom<[u8; 6]> for GameID {
    type Error = std::io::Error;

    fn try_from(value: [u8; 6]) -> Result<Self, Self::Error> {
        let len = value.iter().position(|&b| b == 0).unwrap_or(6);

        if len != 4 && len != 6 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid game id",
            ));
        }

        if !value[..len]
            .iter()
            .all(|&b| b.is_ascii_uppercase() || b.is_ascii_digit())
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid game id",
            ));
        }

        Ok(GameID { inner: value })
    }
}

impl std::str::FromStr for GameID {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buf = [0; 6];
        buf.iter_mut().zip(s.bytes()).for_each(|(b, c)| *b = c);
        GameID::try_from(buf)
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
