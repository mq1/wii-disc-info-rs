// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#![warn(clippy::all, rust_2018_idioms)]

mod formats;
mod gcz;
mod regions;

pub use formats::Format;
pub use regions::RegionCode;
use std::io::{self, Read};

const HEADER_SIZE: usize = 0x60;
const WII_MAGIC: [u8; 4] = [0x5D, 0x1C, 0x9E, 0xA3];
const GC_MAGIC: [u8; 4] = [0xC2, 0x33, 0x9F, 0x3D];
const BUF_SIZE: usize = 0x8000; // 32 KiB

#[derive(Debug, Clone, Copy)]
pub struct Meta {
    format: Format,
    header: [u8; HEADER_SIZE],
    game_id_len: u8,
    game_title_len: u8,
    is_wii: bool,
}

impl Meta {
    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let buf = {
            let mut buf = Box::new_uninit_slice(BUF_SIZE);
            let buf_slice =
                unsafe { std::slice::from_raw_parts_mut(buf.as_mut_ptr().cast(), BUF_SIZE) };
            reader.read_exact(buf_slice)?;
            unsafe { buf.assume_init() }
        };

        let format = Format::parse_header(&buf[..]);
        let header = match format {
            Format::Gcz => gcz::read(reader, &buf[..])?,
            _ => {
                let offset = format.header_offset();
                buf[offset..offset + HEADER_SIZE].try_into().unwrap()
            }
        };

        // Validate Console
        let is_wii = header[0x18..0x1c] == WII_MAGIC;
        let is_gc = header[0x1c..0x20] == GC_MAGIC;
        if is_wii == is_gc {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }

        // Validate Game ID length
        let game_id_len = header[0..6].iter().position(|&b| b == 0).unwrap_or(6);
        if !matches!(game_id_len, 4 | 6) {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }

        // Validate Game ID
        if !header[0..game_id_len]
            .iter()
            .all(|&b| b.is_ascii_uppercase() || b.is_ascii_digit())
        {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }

        // Validate Game Title length
        let game_title_len = header[0x20..].iter().position(|&b| b == 0).unwrap_or(64);
        if game_title_len == 0 {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }

        // Validate Game Title
        if str::from_utf8(&header[0x20..0x20 + game_title_len]).is_err() {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }

        Ok(Self {
            format,
            header,
            game_id_len: game_id_len as u8,
            game_title_len: game_title_len as u8,
            is_wii,
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
        self.is_wii
    }

    pub fn is_gc(&self) -> bool {
        !self.is_wii
    }

    pub fn game_title(&self) -> &str {
        let len = self.game_title_len as usize;
        unsafe { str::from_utf8_unchecked(&self.header[0x20..0x20 + len]) }
    }
}
