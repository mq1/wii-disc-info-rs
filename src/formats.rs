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

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Iso => "ISO",
            Self::Wbfs => "WBFS",
            Self::Ciso => "CISO",
            Self::Rvz => "RVZ",
            Self::Wia => "WIA",
            Self::Tgc => "TGC",
            Self::Gcz => "GCZ",
        };

        f.write_str(name)
    }
}

impl Format {
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
}
