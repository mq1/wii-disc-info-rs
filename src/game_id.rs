// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use crate::{RegionCode, errors::Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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
        unsafe { std::str::from_utf8_unchecked(&self.inner[..len]) }
    }

    #[must_use]
    #[inline]
    pub fn as_partial_str(&self) -> &str {
        // SAFETY: already validated
        unsafe { std::str::from_utf8_unchecked(&self.inner[..3]) }
    }

    #[must_use]
    #[inline]
    pub const fn to_bytes(&self) -> [u8; 6] {
        self.inner
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
}

impl TryFrom<[u8; 6]> for GameID {
    type Error = Error;

    fn try_from(value: [u8; 6]) -> Result<Self, Self::Error> {
        let len = value.iter().position(|&b| b == 0).unwrap_or(6);

        if len != 4 && len != 6 {
            return Err(Error::InvalidGameId);
        }

        if !value[..len]
            .iter()
            .all(|&b| b.is_ascii_uppercase() || b.is_ascii_digit())
        {
            return Err(Error::InvalidGameId);
        }

        Ok(GameID { inner: value })
    }
}

impl std::str::FromStr for GameID {
    type Err = Error;

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
