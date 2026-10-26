// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

// like arrayvec::ArrayString but in safe rust

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayString<const CAP: usize> {
    inner: [u8; CAP],
    len: usize,
}

impl<const CAP: usize> TryFrom<[u8; CAP]> for ArrayString<CAP> {
    type Error = std::str::Utf8Error;

    fn try_from(value: [u8; CAP]) -> Result<Self, Self::Error> {
        let len = value.iter().position(|&b| b == 0).unwrap_or(CAP);

        // validate UTF-8
        let _ = std::str::from_utf8(&value[..len])?;

        Ok(Self { inner: value, len })
    }
}

impl<const CAP: usize> TryFrom<&[u8]> for ArrayString<CAP> {
    type Error = std::str::Utf8Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut buf = [0u8; CAP];
        let len = value.len().min(CAP);
        buf[..len].copy_from_slice(&value[..len]);
        Self::try_from(buf)
    }
}

impl<const CAP: usize> std::str::FromStr for ArrayString<CAP> {
    type Err = std::str::Utf8Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.as_bytes())
    }
}

impl<const CAP: usize> ArrayString<CAP> {
    #[inline]
    #[must_use]
    pub fn zero_filled() -> Self {
        Self {
            inner: [0; CAP],
            len: CAP,
        }
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        // The string is guaranteed to be valid UTF-8
        std::str::from_utf8(&self.inner[..self.len]).unwrap()
    }

    #[inline]
    #[must_use]
    pub fn as_str_until<const N: usize>(&self) -> &str {
        const { assert!(N <= CAP) };

        // The string is guaranteed to be valid UTF-8
        std::str::from_utf8(&self.inner[..N]).unwrap()
    }

    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; CAP] {
        &self.inner
    }

    #[inline]
    #[must_use]
    pub fn to_bytes(&self) -> [u8; CAP] {
        self.inner
    }

    #[inline]
    #[must_use]
    pub fn byte_at<const N: usize>(&self) -> u8 {
        const { assert!(N < CAP) };

        self.inner[N]
    }
}

impl<const CAP: usize> PartialEq<[u8; CAP]> for ArrayString<CAP> {
    fn eq(&self, other: &[u8; CAP]) -> bool {
        &self.inner == other
    }
}
