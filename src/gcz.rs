// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{Error, HEADER_SIZE};
use miniz_oxide::inflate::decompress_to_vec_zlib;
use std::io::{self, Read};

pub fn read<R: Read>(reader: &mut R, header: &mut [u8; HEADER_SIZE]) -> Result<(), Error> {
    // Parse metadata
    let num_blocks = u32::from_le_bytes(header[0x1C..0x20].try_into().unwrap()) as u64;
    let blk0_ptr = u64::from_le_bytes(header[0x20..0x28].try_into().unwrap());
    let blk1_ptr = u64::from_le_bytes(header[0x28..0x30].try_into().unwrap());

    // Calculate compressed size
    let blk0_offset = blk0_ptr & !(1u64 << 63);
    let blk1_offset = blk1_ptr & !(1u64 << 63);
    let compressed_size = (blk1_offset - blk0_offset) as usize;

    // Skip to the beginning of block 0
    let mut skip = (num_blocks - 8) * 8 + num_blocks * 4;
    if blk0_offset > 0 {
        skip += blk0_offset;
    }
    io::copy(&mut reader.take(skip), &mut io::sink())?;

    // Read and Decompress
    let mut block_data = vec![0u8; compressed_size];
    reader.read_exact(&mut block_data)?;

    let decompressed = if blk0_ptr & (1u64 << 63) != 0 {
        block_data // Uncompressed
    } else {
        decompress_to_vec_zlib(&block_data).map_err(|_| Error::GczDecompressionFailed)?
    };

    // Copy to Header Buffer
    header.copy_from_slice(&decompressed[..HEADER_SIZE]);

    Ok(())
}
