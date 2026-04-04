// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    borrow::Cow,
    io::{self, Read},
};

const WII_MAGIC: [u8; 4] = [0x5D, 0x1C, 0x9E, 0xA3];
const GC_MAGIC: [u8; 4] = [0xC2, 0x33, 0x9F, 0x3D];

pub struct Meta {
    game_id: [u8; 6],
    disc_number: u8,
    disc_version: u8,
    wii_magic: [u8; 4],
    gc_magic: [u8; 4],
    game_title: [u8; 0x40],
}

impl Meta {
    pub fn game_id(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.game_id)
    }

    pub fn disc_number(&self) -> u8 {
        self.disc_number
    }

    pub fn disc_version(&self) -> u8 {
        self.disc_version
    }

    pub fn is_wii(&self) -> bool {
        self.wii_magic == WII_MAGIC
    }

    pub fn is_gc(&self) -> bool {
        self.gc_magic == GC_MAGIC
    }

    pub fn game_title(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.game_title)
    }
}

impl Meta {
    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let game_id = {
            let mut buf = [0; 6];
            reader.read_exact(&mut buf)?;
            buf
        };

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
            buf
        };

        let meta = Self {
            game_id,
            disc_number,
            disc_version,
            wii_magic,
            gc_magic,
            game_title,
        };

        Ok(meta)
    }
}

/// Reads the disc header from a Wii/GameCube disc image (ISO or WBFS or CISO)
pub fn query<R: Read>(reader: &mut R) -> io::Result<Meta> {
    let mut meta = Meta::read(reader)?;

    if meta.game_id.starts_with(b"WBFS") {
        io::copy(&mut reader.take(0x1a0), &mut io::sink())?;
        meta = Meta::read(reader)?;
    } else if meta.game_id.starts_with(b"CISO") {
        io::copy(&mut reader.take(0x7fa0), &mut io::sink())?;
        meta = Meta::read(reader)?;
    }

    if !meta.is_wii() && !meta.is_gc() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Not a valid Wii or GameCube disc image",
        ));
    }

    Ok(meta)
}
