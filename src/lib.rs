// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod formats;
pub mod game_id;
mod gcz;
pub mod regions;
mod util;

use crate::{formats::HeaderOffset, game_id::GameID};
pub use formats::Format;
pub use regions::RegionCode;
use util::ArrayString;

const HEADER_SIZE: usize = 0x60;
const WII_MAGIC: [u8; 4] = [0x5D, 0x1C, 0x9E, 0xA3];
const GC_MAGIC: [u8; 4] = [0xC2, 0x33, 0x9F, 0x3D];
const BUF_SIZE: usize = 0x8000; // 32 KiB

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Meta {
    format: Format,
    game_id: GameID,
    game_title: ArrayString<0x40>,
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

        let game_id = GameID::try_from(&header[0..6])?;

        let game_title = ArrayString::try_from(&header[0x20..0x60])
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        // Validate Game Title length
        if game_title.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "game title is empty",
            ));
        }

        let disc_number = header[6];
        let disc_version = header[7];

        Ok(Self {
            format: Format::Iso,
            game_id,
            game_title,
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
        self.game_title.as_str()
    }
}
