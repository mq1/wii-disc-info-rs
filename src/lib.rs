// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#![warn(clippy::all, rust_2018_idioms)]

mod errors;
mod formats;
mod gcz;
mod regions;

pub use errors::Error;
pub use formats::Format;
pub use regions::RegionCode;
use std::io::{self, Read};

const HEADER_SIZE: usize = 0x60;
const WII_MAGIC: [u8; 4] = [0x5D, 0x1C, 0x9E, 0xA3];
const GC_MAGIC: [u8; 4] = [0xC2, 0x33, 0x9F, 0x3D];

#[derive(Debug, Clone, Copy)]
pub struct Meta {
    format: Format,
    header: [u8; HEADER_SIZE],
    game_id_len: u8,
    game_title_len: u8,
}

impl Meta {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut header = [0; HEADER_SIZE];
        reader.read_exact(&mut header)?;

        let format = Format::from(&header);
        let header_pos = format.header_pos();

        // Skip to the header position without using Seek
        if header_pos > 0 {
            if HEADER_SIZE > header_pos {
                let overlap = HEADER_SIZE - header_pos;
                header.copy_within(header_pos.., 0);
                reader.read_exact(&mut header[overlap..])?;
            } else {
                let skip = (header_pos - HEADER_SIZE) as u64;
                io::copy(&mut reader.take(skip), &mut io::sink())?;
                reader.read_exact(&mut header)?;
            }
        }

        // Decompress header if GCZ
        if format == Format::Gcz {
            gcz::read(reader, &mut header)?;
        }

        // Validate Console
        let is_wii = header[0x18..0x1c] == WII_MAGIC;
        let is_gc = header[0x1c..0x20] == GC_MAGIC;
        if is_wii == is_gc {
            return Err(Error::InvalidConsole);
        }

        // Validate Game ID length
        let game_id_len = header[0..6].iter().position(|&b| b == 0).unwrap_or(6);
        if !matches!(game_id_len, 4 | 6) {
            return Err(Error::InvalidGameId);
        }

        // Validate Game ID
        if !header[0..game_id_len]
            .iter()
            .all(|&b| b.is_ascii_uppercase() || b.is_ascii_digit())
        {
            return Err(Error::InvalidGameId);
        }

        // Validate Game Title length
        let game_title_len = header[0x20..].iter().position(|&b| b == 0).unwrap_or(64);
        if game_title_len == 0 {
            return Err(Error::InvalidGameTitle);
        }

        // Validate Game Title
        if str::from_utf8(&header[0x20..0x20 + game_title_len]).is_err() {
            return Err(Error::InvalidGameTitle);
        }

        Ok(Self {
            format,
            header,
            game_id_len: game_id_len as u8,
            game_title_len: game_title_len as u8,
        })
    }

    pub fn format(&self) -> Format {
        self.format
    }

    pub fn game_id(&self) -> &str {
        let len = self.game_id_len as usize;
        unsafe { str::from_utf8_unchecked(&self.header[0..len]) }
    }

    pub fn region(&self) -> RegionCode {
        // Ratatouille (RLWW78) has a region byte of 'W', but it's actually a Scandinavian release
        if self.header[0..6] == *b"RLWW78" {
            return RegionCode::Scandinavia;
        }

        RegionCode::from(self.header[3])
    }

    pub fn disc_number(&self) -> u8 {
        self.header[6]
    }

    pub fn disc_version(&self) -> u8 {
        self.header[7]
    }

    pub fn is_wii(&self) -> bool {
        self.header[0x18..0x1c] == WII_MAGIC
    }

    pub fn is_gc(&self) -> bool {
        self.header[0x1c..0x20] == GC_MAGIC
    }

    pub fn game_title(&self) -> &str {
        let len = self.game_title_len as usize;
        unsafe { str::from_utf8_unchecked(&self.header[0x20..0x20 + len]) }
    }
}
