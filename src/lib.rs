// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#![warn(clippy::all, rust_2018_idioms)]

use std::io::Read;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidGameId,
    InvalidGameTitle,
    InvalidConsole,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {e}"),
            Self::InvalidGameId => f.write_str("invalid game ID"),
            Self::InvalidGameTitle => f.write_str("invalid game title"),
            Self::InvalidConsole => f.write_str("invalid console"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

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
    pub fn initial_padding(self) -> Option<u64> {
        match self {
            Format::Wbfs => Some(0x200 - 0x6),
            Format::Ciso | Format::Tgc => Some(0x8000 - 0x6),
            Format::Rvz | Format::Wia => Some(0x58 - 0x6),
            Format::Iso => None,
        }
    }
}

impl From<[u8; 4]> for Format {
    fn from(magic: [u8; 4]) -> Self {
        match magic {
            [b'W', b'B', b'F', b'S'] => Self::Wbfs,
            [b'C', b'I', b'S', b'O'] => Self::Ciso,
            [b'R', b'V', b'Z', 0x01] => Self::Rvz,
            [b'W', b'I', b'A', 0x01] => Self::Wia,
            [0xae, 0x0f, 0x38, 0xa2] => Self::Tgc,
            _ => Self::Iso,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionCode {
    SystemWiiChannels,
    UfouriaTheSagaNA,
    Germany,
    USA,
    France,
    NetherlandsEuropeAlternateLanguages,
    Italy,
    Japan,
    Korea,
    JapaneseImportToEuropeAustraliaAndOtherPALRegions,
    AmericanImportToEuropeAustraliaAndOtherPALRegions,
    JapaneseImportToUSAAndOtherNTSCRegions,
    EuropeAndOtherPALRegionsSuchAsAustralia,
    JapaneseVirtualConsoleImportToKorea,
    Russia,
    Spain,
    AmericanVirtualConsoleImportToKorea,
    AustraliaEuropeAlternateLanguages,
    Scandinavia,
    RepublicOfChinaTaiwanHongKongMacau,
    EuropeAlternateLanguagesUSSpecialReleases,
    Unknown,
}

impl std::fmt::Display for RegionCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::SystemWiiChannels => "System Wii Channels (i.e. Mii Channel)",
            Self::UfouriaTheSagaNA => "Ufouria: The Saga (NA)",
            Self::Germany => "Germany",
            Self::USA => "USA",
            Self::France => "France",
            Self::NetherlandsEuropeAlternateLanguages => "Netherlands / Europe alternate languages",
            Self::Italy => "Italy",
            Self::Japan => "Japan",
            Self::Korea => "Korea",
            Self::JapaneseImportToEuropeAustraliaAndOtherPALRegions => {
                "Japanese import to Europe, Australia and other PAL regions"
            }
            Self::AmericanImportToEuropeAustraliaAndOtherPALRegions => {
                "American import to Europe, Australia and other PAL regions"
            }
            Self::JapaneseImportToUSAAndOtherNTSCRegions => {
                "Japanese import to USA and other NTSC regions"
            }
            Self::EuropeAndOtherPALRegionsSuchAsAustralia => {
                "Europe and other PAL regions such as Australia"
            }
            Self::JapaneseVirtualConsoleImportToKorea => "Japanese Virtual Console import to Korea",
            Self::Russia => "Russia",
            Self::Spain => "Spain",
            Self::AmericanVirtualConsoleImportToKorea => "American Virtual Console import to Korea",
            Self::AustraliaEuropeAlternateLanguages => "Australia / Europe alternate languages",
            Self::Scandinavia => "Scandinavia",
            Self::RepublicOfChinaTaiwanHongKongMacau => {
                "Republic of China (Taiwan) / Hong Kong / Macau"
            }
            Self::EuropeAlternateLanguagesUSSpecialReleases => {
                "Europe alternate languages / US special releases"
            }
            Self::Unknown => "Unknown",
        };

        f.write_str(name)
    }
}

impl From<u8> for RegionCode {
    fn from(b: u8) -> Self {
        match b {
            b'A' => Self::SystemWiiChannels,
            b'B' => Self::UfouriaTheSagaNA,
            b'D' => Self::Germany,
            b'E' => Self::USA,
            b'F' => Self::France,
            b'H' => Self::NetherlandsEuropeAlternateLanguages,
            b'I' => Self::Italy,
            b'J' => Self::Japan,
            b'K' => Self::Korea,
            b'L' => Self::JapaneseImportToEuropeAustraliaAndOtherPALRegions,
            b'M' => Self::AmericanImportToEuropeAustraliaAndOtherPALRegions,
            b'N' => Self::JapaneseImportToUSAAndOtherNTSCRegions,
            b'P' => Self::EuropeAndOtherPALRegionsSuchAsAustralia,
            b'Q' => Self::JapaneseVirtualConsoleImportToKorea,
            b'R' => Self::Russia,
            b'S' => Self::Spain,
            b'T' => Self::AmericanVirtualConsoleImportToKorea,
            b'U' => Self::AustraliaEuropeAlternateLanguages,
            b'V' => Self::Scandinavia,
            b'W' => Self::RepublicOfChinaTaiwanHongKongMacau,
            b'X' | b'Y' | b'Z' => Self::EuropeAlternateLanguagesUSSpecialReleases,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Meta {
    format: Format,
    game_id: [u8; 6],
    game_id_length: usize,
    disc_number: u8,
    disc_version: u8,
    wii_magic: [u8; 4],
    gc_magic: [u8; 4],
    game_title: [u8; 64],
    game_title_length: usize,
}

impl Meta {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut game_id = [0; 6];
        reader.read_exact(&mut game_id)?;

        let format = {
            let mut buf = [0; 4];
            buf.copy_from_slice(&game_id[..4]);
            Format::from(buf)
        };

        if let Some(padding) = format.initial_padding() {
            std::io::copy(&mut reader.take(padding), &mut std::io::sink())?;
            reader.read_exact(&mut game_id)?;
        }

        let game_id_length = game_id.iter().position(|&b| b == 0).unwrap_or(6);
        if !matches!(game_id_length, 4 | 6) {
            return Err(Error::InvalidGameId);
        }

        // Check if game_id is uppercase alphanumeric
        for b in &game_id[..game_id_length] {
            if !matches!(b, b'A'..=b'Z' | b'0'..=b'9') {
                return Err(Error::InvalidGameId);
            }
        }

        let disc_number = {
            let mut buf = [0; 1];
            reader.read_exact(&mut buf)?;
            buf[0]
        };

        let disc_version = {
            let mut buf = [0; 1];
            reader.read_exact(&mut buf)?;
            buf[0]
        };

        // padding
        std::io::copy(&mut reader.take(0x10), &mut std::io::sink())?;

        let wii_magic = {
            let mut buf = [0; 4];
            reader.read_exact(&mut buf)?;
            buf
        };

        let gc_magic = {
            let mut buf = [0; 4];
            reader.read_exact(&mut buf)?;
            buf
        };

        let mut game_title = [0; 64];
        reader.read_exact(&mut game_title)?;
        let game_title_length = game_title.iter().position(|&b| b == 0).unwrap_or(64);
        if str::from_utf8(&game_title[..game_title_length]).is_err() {
            return Err(Error::InvalidGameTitle);
        }

        let meta = Self {
            format,
            game_id,
            game_id_length,
            disc_number,
            disc_version,
            wii_magic,
            gc_magic,
            game_title,
            game_title_length,
        };

        if !meta.is_wii() && !meta.is_gc() {
            return Err(Error::InvalidConsole);
        }

        Ok(meta)
    }

    pub fn format(&self) -> Format {
        self.format
    }

    pub fn game_id(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.game_id[..self.game_id_length]) }
    }

    pub fn region(&self) -> RegionCode {
        // Ratatouille (RLWW78) has a region byte of 'W', but it's actually a Scandinavian release
        if self.game_id == [b'R', b'L', b'W', b'W', b'7', b'8'] {
            return RegionCode::Scandinavia;
        }

        RegionCode::from(self.game_id[3])
    }

    pub fn disc_number(&self) -> u8 {
        self.disc_number
    }

    pub fn disc_version(&self) -> u8 {
        self.disc_version
    }

    pub fn is_wii(&self) -> bool {
        self.wii_magic == [0x5D, 0x1C, 0x9E, 0xA3]
    }

    pub fn is_gc(&self) -> bool {
        self.gc_magic == [0xC2, 0x33, 0x9F, 0x3D]
    }

    pub fn game_title(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.game_title[..self.game_title_length]) }
    }
}
