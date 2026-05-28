// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#![warn(clippy::all, rust_2018_idioms)]

mod errors;
mod formats;
mod regions;

pub use errors::Error;
pub use formats::Format;
pub use regions::RegionCode;
use std::io::Read;

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
