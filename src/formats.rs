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
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Iso => "ISO",
            Self::Wbfs => "WBFS",
            Self::Ciso => "CISO",
            Self::Rvz => "RVZ",
            Self::Wia => "WIA",
            Self::Tgc => "TGC",
        };

        f.write_str(name)
    }
}

impl Format {
    pub fn header_pos(self) -> usize {
        match self {
            Format::Wbfs => 0x200,
            Format::Ciso | Format::Tgc => 0x8000,
            Format::Rvz | Format::Wia => 0x58,
            Format::Iso => 0,
        }
    }
}

impl From<&[u8; 96]> for Format {
    fn from(header: &[u8; 96]) -> Self {
        match header[..4] {
            [b'W', b'B', b'F', b'S'] => Self::Wbfs,
            [b'C', b'I', b'S', b'O'] => Self::Ciso,
            [b'R', b'V', b'Z', 0x01] => Self::Rvz,
            [b'W', b'I', b'A', 0x01] => Self::Wia,
            [0xae, 0x0f, 0x38, 0xa2] => Self::Tgc,
            _ => Self::Iso,
        }
    }
}
