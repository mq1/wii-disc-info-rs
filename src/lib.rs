// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#![warn(clippy::all, rust_2018_idioms)]
#![deny(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod formats;
pub mod game_id;
mod gcz;
pub mod regions;

use crate::{formats::HeaderOffset, game_id::GameID};
pub use formats::Format;
pub use regions::RegionCode;

const HEADER_SIZE: usize = 0x60;
const WII_MAGIC: [u8; 4] = [0x5D, 0x1C, 0x9E, 0xA3];
const GC_MAGIC: [u8; 4] = [0xC2, 0x33, 0x9F, 0x3D];
const BUF_SIZE: usize = 0x8000; // 32 KiB

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Meta {
    format: Format,
    game_id: GameID,
    game_title: [u8; 0x40],
    game_title_len: u8,
    is_wii: bool,
    disc_number: u8,
    disc_version: u8,
}

impl Meta {
    fn parse_header(header: [u8; HEADER_SIZE]) -> std::io::Result<Self> {
        // Validate Console
        let is_wii = header[0x18..0x1c] == WII_MAGIC;
        let is_gc = header[0x1c..0x20] == GC_MAGIC;
        if is_wii == is_gc {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid console",
            ));
        }

        let game_id: [u8; 6] = header[0..6].try_into().unwrap();
        let game_id = GameID::try_from(game_id)?;

        let game_title: [u8; 0x40] = header[0x20..0x60].try_into().unwrap();

        // Validate Game Title length
        let game_title_len = game_title.iter().position(|&b| b == 0).unwrap_or(0x40) as u8;
        if game_title_len == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid game title",
            ));
        }

        // Validate Game Title
        if std::str::from_utf8(&game_title).is_err() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid game title",
            ));
        }

        let disc_number = header[6];
        let disc_version = header[7];

        Ok(Self {
            format: Format::Iso,
            game_id,
            game_title,
            game_title_len,
            is_wii,
            disc_number,
            disc_version,
        })
    }

    /// Synchronous read
    pub fn read(mut reader: impl std::io::Read) -> std::io::Result<Self> {
        let mut buf = vec![0u8; BUF_SIZE];
        reader.read_exact(&mut buf)?;

        let format = Format::parse_header(&buf[..])?;
        let header = match format.header_offset() {
            HeaderOffset::Fixed(offset) if offset < BUF_SIZE => {
                buf[offset..offset + HEADER_SIZE].try_into().unwrap()
            }
            HeaderOffset::Fixed(offset) if offset == BUF_SIZE => {
                reader.read_exact(&mut buf)?;
                buf[..HEADER_SIZE].try_into().unwrap()
            }
            HeaderOffset::Fixed(_) => unreachable!(),
            HeaderOffset::Gcz => gcz::read(&mut reader, &buf[..])?,
        };

        let meta = Self {
            format,
            ..Self::parse_header(header)?
        };

        Ok(meta)
    }

    /// Asynchronous read
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    #[cfg(feature = "async")]
    pub async fn read_async(
        mut reader: impl futures_lite::AsyncReadExt + Unpin,
    ) -> std::io::Result<Self> {
        let mut buf = vec![0u8; BUF_SIZE];
        reader.read_exact(&mut buf).await?;

        let format = Format::parse_header(&buf[..])?;
        let header = match format.header_offset() {
            HeaderOffset::Fixed(offset) if offset < BUF_SIZE => {
                buf[offset..offset + HEADER_SIZE].try_into().unwrap()
            }
            HeaderOffset::Fixed(offset) if offset == BUF_SIZE => {
                reader.read_exact(&mut buf).await?;
                buf[..HEADER_SIZE].try_into().unwrap()
            }
            HeaderOffset::Fixed(_) => unreachable!(),
            HeaderOffset::Gcz => gcz::read_async(&mut reader, &buf[..]).await?,
        };

        let meta = Self {
            format,
            ..Self::parse_header(header)?
        };

        Ok(meta)
    }

    #[must_use]
    #[inline]
    pub fn format(&self) -> Format {
        self.format
    }

    #[must_use]
    #[inline]
    pub fn game_id(&self) -> GameID {
        self.game_id
    }

    #[must_use]
    #[inline]
    pub fn disc_number(&self) -> u8 {
        self.disc_number
    }

    #[must_use]
    #[inline]
    pub fn disc_version(&self) -> u8 {
        self.disc_version
    }

    #[must_use]
    #[inline]
    pub fn is_wii(&self) -> bool {
        self.is_wii
    }

    #[must_use]
    #[inline]
    pub fn is_gc(&self) -> bool {
        !self.is_wii
    }

    #[must_use]
    #[inline]
    pub fn game_title(&self) -> &str {
        let len = usize::from(self.game_title_len);

        // SAFETY: game_title is validated in Meta::read
        #[allow(unsafe_code)]
        unsafe {
            str::from_utf8_unchecked(self.game_title.get_unchecked(..len))
        }
    }
}
