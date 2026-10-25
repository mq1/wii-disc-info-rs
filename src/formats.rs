// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderOffset {
    Fixed(usize),
    Gcz,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Iso,
    Wbfs,
    Ciso,
    Rvz,
    Wia,
    Tgc,
    Gcz,
}

impl Format {
    #[must_use]
    pub fn parse_header(header: &[u8]) -> std::io::Result<Self> {
        match header.get(..4) {
            Some([b'W', b'B', b'F', b'S']) => Ok(Self::Wbfs),
            Some([b'C', b'I', b'S', b'O']) => Ok(Self::Ciso),
            Some([b'R', b'V', b'Z', 0x01]) => Ok(Self::Rvz),
            Some([b'W', b'I', b'A', 0x01]) => Ok(Self::Wia),
            Some([0xae, 0x0f, 0x38, 0xa2]) => Ok(Self::Tgc),
            Some([0x01, 0xc0, 0x0b, 0xb1]) => Ok(Self::Gcz),
            Some(_) => Ok(Self::Iso),
            None => Err(std::io::ErrorKind::UnexpectedEof.into()),
        }
    }

    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Format::Iso => "ISO",
            Format::Wbfs => "WBFS",
            Format::Ciso => "CISO",
            Format::Rvz => "RVZ",
            Format::Wia => "WIA",
            Format::Tgc => "TGC",
            Format::Gcz => "GCZ",
        }
    }

    #[must_use]
    pub const fn header_offset(&self) -> HeaderOffset {
        match self {
            Format::Iso => HeaderOffset::Fixed(0),
            Format::Wbfs => HeaderOffset::Fixed(0x200),
            Format::Rvz | Format::Wia => HeaderOffset::Fixed(0x58),
            Format::Ciso | Format::Tgc => HeaderOffset::Fixed(0x8000),
            Format::Gcz => HeaderOffset::Gcz,
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl AsRef<str> for Format {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
