// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use arrayvec::ArrayString;
use std::{
    fmt,
    io::{self, Read},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Iso,
    Wbfs,
    Ciso,
    Rvz,
    Wia,
    Tgc,
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

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Iso => write!(f, "ISO"),
            Self::Wbfs => write!(f, "WBFS"),
            Self::Ciso => write!(f, "CISO"),
            Self::Rvz => write!(f, "RVZ"),
            Self::Wia => write!(f, "WIA"),
            Self::Tgc => write!(f, "TGC"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionChar {
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
    Unknown(char),
}

impl From<char> for RegionChar {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::SystemWiiChannels,
            'B' => Self::UfouriaTheSagaNA,
            'D' => Self::Germany,
            'E' => Self::USA,
            'F' => Self::France,
            'H' => Self::NetherlandsEuropeAlternateLanguages,
            'I' => Self::Italy,
            'J' => Self::Japan,
            'K' => Self::Korea,
            'L' => Self::JapaneseImportToEuropeAustraliaAndOtherPALRegions,
            'M' => Self::AmericanImportToEuropeAustraliaAndOtherPALRegions,
            'N' => Self::JapaneseImportToUSAAndOtherNTSCRegions,
            'P' => Self::EuropeAndOtherPALRegionsSuchAsAustralia,
            'Q' => Self::JapaneseVirtualConsoleImportToKorea,
            'R' => Self::Russia,
            'S' => Self::Spain,
            'T' => Self::AmericanVirtualConsoleImportToKorea,
            'U' => Self::AustraliaEuropeAlternateLanguages,
            'V' => Self::Scandinavia,
            'W' => Self::RepublicOfChinaTaiwanHongKongMacau,
            'X' | 'Y' | 'Z' => Self::EuropeAlternateLanguagesUSSpecialReleases,
            c => Self::Unknown(c),
        }
    }
}

impl fmt::Display for RegionChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SystemWiiChannels => write!(f, "System Wii Channels (i.e. Mii Channel)"),
            Self::UfouriaTheSagaNA => write!(f, "Ufouria: The Saga (NA)"),
            Self::Germany => write!(f, "Germany"),
            Self::USA => write!(f, "USA"),
            Self::France => write!(f, "France"),
            Self::NetherlandsEuropeAlternateLanguages => {
                write!(f, "Netherlands / Europe alternate languages")
            }
            Self::Italy => write!(f, "Italy"),
            Self::Japan => write!(f, "Japan"),
            Self::Korea => write!(f, "Korea"),
            Self::JapaneseImportToEuropeAustraliaAndOtherPALRegions => write!(
                f,
                "Japanese import to Europe, Australia and other PAL regions"
            ),
            Self::AmericanImportToEuropeAustraliaAndOtherPALRegions => write!(
                f,
                "American import to Europe, Australia and other PAL regions"
            ),
            Self::JapaneseImportToUSAAndOtherNTSCRegions => {
                write!(f, "Japanese import to USA and other NTSC regions")
            }
            Self::EuropeAndOtherPALRegionsSuchAsAustralia => {
                write!(f, "Europe and other PAL regions such as Australia")
            }
            Self::JapaneseVirtualConsoleImportToKorea => {
                write!(f, "Japanese Virtual Console import to Korea")
            }
            Self::Russia => write!(f, "Russia"),
            Self::Spain => write!(f, "Spain"),
            Self::AmericanVirtualConsoleImportToKorea => {
                write!(f, "American Virtual Console import to Korea")
            }
            Self::AustraliaEuropeAlternateLanguages => {
                write!(f, "Australia / Europe alternate languages")
            }
            Self::Scandinavia => write!(f, "Scandinavia"),
            Self::RepublicOfChinaTaiwanHongKongMacau => {
                write!(f, "Republic of China (Taiwan) / Hong Kong / Macau")
            }
            Self::EuropeAlternateLanguagesUSSpecialReleases => {
                write!(f, "Europe alternate languages / US special releases")
            }
            Self::Unknown(byte) => write!(f, "Unknown ({})", char::from(*byte)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Meta {
    format: Format,
    game_id: ArrayString<6>,
    disc_number: u8,
    disc_version: u8,
    wii_magic: [u8; 4],
    gc_magic: [u8; 4],
    game_title: ArrayString<0x40>,
}

impl Meta {
    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut game_id = [0; 6];
        reader.read_exact(&mut game_id)?;

        let format = {
            let mut buf = [0; 4];
            buf.copy_from_slice(&game_id[..4]);
            Format::from(buf)
        };

        if let Some(padding) = format.initial_padding() {
            io::copy(&mut reader.take(padding), &mut io::sink())?;
            reader.read_exact(&mut game_id)?;
        }

        let game_id = ArrayString::from_byte_string(&game_id).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Game ID is not valid UTF-8")
        })?;

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
        io::copy(&mut reader.take(0x10), &mut io::sink())?;

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

        let game_title = {
            let mut buf = [0; 0x40];
            reader.read_exact(&mut buf)?;
            ArrayString::from_byte_string(&buf).map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "Game title is not valid UTF-8")
            })?
        };

        let meta = Self {
            format,
            game_id,
            disc_number,
            disc_version,
            wii_magic,
            gc_magic,
            game_title,
        };

        if !meta.is_wii() && !meta.is_gc() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not a valid Wii or GameCube disc image",
            ));
        }

        Ok(meta)
    }

    pub fn format(&self) -> Format {
        self.format
    }

    pub fn game_id(&self) -> &str {
        &self.game_id
    }

    pub fn region(&self) -> RegionChar {
        // Ratatouille (RLWW78) has a region byte of 'W', but it's actually a Scandinavian release
        if self.game_id.eq("RLWW78") {
            return RegionChar::Scandinavia;
        }

        let region_char = self.game_id.chars().nth(3).unwrap_or('\0');
        RegionChar::from(region_char)
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
        &self.game_title
    }
}
