// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#![warn(clippy::all, rust_2018_idioms)]

mod errors;
mod formats;
mod regions;

pub use errors::Error;
pub use formats::Format;
pub use regions::RegionCode;
use std::io::{self, Read};

const HEADER_SIZE: usize = 0x60;

#[derive(Debug, Clone, Copy)]
pub struct Meta {
    format: Format,
    header: [u8; HEADER_SIZE],
    game_id_len: usize,
    game_title_len: usize,
}

impl Meta {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut header = [0; HEADER_SIZE];
        reader.read_exact(&mut header)?;

        let format = Format::from(&header);

        if let Some(pos) = format.header_pos() {
            if HEADER_SIZE > pos {
                let overflow = HEADER_SIZE - pos;
                header.copy_within(pos.., 0);
                reader.read_exact(&mut header[overflow..])?;
            } else {
                let skip = (pos - HEADER_SIZE) as u64;
                io::copy(&mut reader.take(skip), &mut io::sink())?;
                reader.read_exact(&mut header)?;
            }
        }

        // Check for game_id len
        let game_id_len = header[0..6].iter().position(|&b| b == 0).unwrap_or(6);
        if !matches!(game_id_len, 4 | 6) {
            return Err(Error::InvalidGameId);
        }

        // Check if game_id is uppercase alphanumeric
        if header[0..game_id_len]
            .iter()
            .any(|&b| !b.is_ascii_uppercase() && !b.is_ascii_digit())
        {
            return Err(Error::InvalidGameId);
        }

        // check for game title len
        let game_title_len = header[0x20..].iter().position(|&b| b == 0).unwrap_or(64);
        if game_title_len == 0 {
            return Err(Error::InvalidGameTitle);
        }

        // check for game title validity
        if str::from_utf8(&header[0x20..0x20 + game_title_len]).is_err() {
            return Err(Error::InvalidGameTitle);
        }

        let meta = Self {
            format,
            header,
            game_id_len,
            game_title_len,
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
        unsafe { str::from_utf8_unchecked(&self.header[0..self.game_id_len]) }
    }

    pub fn region(&self) -> RegionCode {
        // Ratatouille (RLWW78) has a region byte of 'W', but it's actually a Scandinavian release
        if self.header[0..6] == [b'R', b'L', b'W', b'W', b'7', b'8'] {
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
        self.header[0x18..0x1c] == [0x5D, 0x1C, 0x9E, 0xA3]
    }

    pub fn is_gc(&self) -> bool {
        self.header[0x1c..0x20] == [0xC2, 0x33, 0x9F, 0x3D]
    }

    pub fn game_title(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.header[0x20..0x20 + self.game_title_len]) }
    }
}
