// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#![warn(clippy::all, rust_2018_idioms)]

pub mod errors;
pub mod formats;
pub mod game_id;
mod gcz;
pub mod regions;

use crate::{errors::Error, game_id::GameID};
pub use formats::Format;
use futures::{AsyncRead, AsyncReadExt};
pub use regions::RegionCode;

const HEADER_SIZE: usize = 0x60;
const WII_MAGIC: [u8; 4] = [0x5D, 0x1C, 0x9E, 0xA3];
const GC_MAGIC: [u8; 4] = [0xC2, 0x33, 0x9F, 0x3D];
const BUF_SIZE: usize = 0x8000; // 32 KiB

#[derive(Debug, Clone, Copy)]
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
    pub async fn read<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = vec![0u8; BUF_SIZE].into_boxed_slice();

        reader.read_exact(&mut buf).await?;

        let format = Format::parse_header(&buf[..]);
        let header = match format {
            Format::Iso => buf[0..HEADER_SIZE].try_into().unwrap(),
            Format::Wbfs => buf[0x200..0x200 + HEADER_SIZE].try_into().unwrap(),
            Format::Rvz | Format::Wia => buf[0x58..0x58 + HEADER_SIZE].try_into().unwrap(),
            Format::Ciso | Format::Tgc => {
                reader.read_exact(&mut buf).await?;
                buf[0..HEADER_SIZE].try_into().unwrap()
            }
            Format::Gcz => gcz::read(reader, &buf[..]).await?,
        };

        // Validate Console
        let is_wii = header[0x18..0x1c] == WII_MAGIC;
        let is_gc = header[0x1c..0x20] == GC_MAGIC;
        if is_wii == is_gc {
            return Err(Error::InvalidConsole);
        }

        let game_id: [u8; 6] = header[0..6].try_into().unwrap();
        let game_id = GameID::try_from(game_id)?;

        let game_title: [u8; 0x40] = header[0x20..0x60].try_into().unwrap();

        // Validate Game Title length
        let game_title_len = game_title.iter().position(|&b| b == 0).unwrap_or(0x40) as u8;
        if game_title_len == 0 {
            return Err(Error::InvalidGameTitle);
        }

        // Validate Game Title
        if std::str::from_utf8(&game_title).is_err() {
            return Err(Error::InvalidGameTitle);
        };

        let disc_number = header[6];
        let disc_version = header[7];

        Ok(Self {
            format,
            game_id,
            game_title,
            game_title_len,
            is_wii,
            disc_number,
            disc_version,
        })
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
        let len = self.game_title_len as usize;

        // SAFETY: game_title is validated in Meta::read
        unsafe { str::from_utf8_unchecked(&self.game_title[0..len]) }
    }
}
