// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

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
    pub fn parse_header(header: &[u8]) -> Self {
        match header[0..4] {
            [b'W', b'B', b'F', b'S'] => Self::Wbfs,
            [b'C', b'I', b'S', b'O'] => Self::Ciso,
            [b'R', b'V', b'Z', 0x01] => Self::Rvz,
            [b'W', b'I', b'A', 0x01] => Self::Wia,
            [0xae, 0x0f, 0x38, 0xa2] => Self::Tgc,
            [0x01, 0xc0, 0x0b, 0xb1] => Self::Gcz,
            _ => Self::Iso,
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &'static str {
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
