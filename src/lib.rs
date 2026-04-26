// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#![warn(clippy::all, rust_2018_idioms)]

use arrayvec::ArrayString;
use derive_more::Display;
use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Format {
    #[display("ISO")]
    Iso,

    #[display("WBFS")]
    Wbfs,

    #[display("CISO")]
    Ciso,

    #[display("RVZ")]
    Rvz,

    #[display("WIA")]
    Wia,

    #[display("TGC")]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum RegionCode {
    #[display("System Wii Channels (i.e. Mii Channel)")]
    SystemWiiChannels,

    #[display("Ufouria: The Saga (NA)")]
    UfouriaTheSagaNA,

    #[display("Germany")]
    Germany,

    #[display("USA")]
    USA,

    #[display("France")]
    France,

    #[display("Netherlands / Europe alternate languages")]
    NetherlandsEuropeAlternateLanguages,

    #[display("Italy")]
    Italy,

    #[display("Japan")]
    Japan,

    #[display("Korea")]
    Korea,

    #[display("Japanese import to Europe, Australia and other PAL regions")]
    JapaneseImportToEuropeAustraliaAndOtherPALRegions,

    #[display("American import to Europe, Australia and other PAL regions")]
    AmericanImportToEuropeAustraliaAndOtherPALRegions,

    #[display("Japanese import to USA and other NTSC regions")]
    JapaneseImportToUSAAndOtherNTSCRegions,

    #[display("Europe and other PAL regions such as Australia")]
    EuropeAndOtherPALRegionsSuchAsAustralia,

    #[display("Japanese Virtual Console import to Korea")]
    JapaneseVirtualConsoleImportToKorea,

    #[display("Russia")]
    Russia,

    #[display("Spain")]
    Spain,

    #[display("American Virtual Console import to Korea")]
    AmericanVirtualConsoleImportToKorea,

    #[display("Australia / Europe alternate languages")]
    AustraliaEuropeAlternateLanguages,

    #[display("Scandinavia")]
    Scandinavia,

    #[display("Republic of China (Taiwan) / Hong Kong / Macau")]
    RepublicOfChinaTaiwanHongKongMacau,

    #[display("Europe alternate languages / US special releases")]
    EuropeAlternateLanguagesUSSpecialReleases,

    #[display("Unknown ({_0})")]
    Unknown(char),
}

impl From<char> for RegionCode {
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

#[derive(Debug, Clone, Copy)]
pub struct Meta {
    format: Format,
    game_id: ArrayString<6>,
    disc_number: u8,
    disc_version: u8,
    wii_magic: [u8; 4],
    gc_magic: [u8; 4],
    game_title: ArrayString<64>,
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

        if game_id.chars().any(|c| !c.is_ascii_alphanumeric()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Game ID contains invalid characters",
            ));
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
            let mut buf = [0; 64];
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

    pub fn game_id(&self) -> &ArrayString<6> {
        &self.game_id
    }

    pub fn region(&self) -> RegionCode {
        // Ratatouille (RLWW78) has a region byte of 'W', but it's actually a Scandinavian release
        if self.game_id.eq("RLWW78") {
            return RegionCode::Scandinavia;
        }

        let region_char = self.game_id.chars().nth(3).unwrap_or('\0');
        RegionCode::from(region_char)
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
