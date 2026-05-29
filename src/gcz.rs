// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{Error, HEADER_SIZE};
use std::io::{self, Read};

pub fn read<R: Read>(reader: &mut R, header: &mut [u8; HEADER_SIZE]) -> Result<(), Error> {
    let num_blocks = u32::from_le_bytes(header[0x1C..0x20].try_into().unwrap());
    if num_blocks == 0 {
        return Err(Error::Gcz);
    }

    // header[0x20..] contains the block pointer table inline
    let blk0_ptr = u64::from_le_bytes(header[0x20..0x28].try_into().unwrap());
    let blk0_offset = blk0_ptr & !(1u64 << 63);
    // Block 0 must be at offset 0 in the data region
    if blk0_offset != 0 {
        return Err(Error::Gcz);
    }

    // Determine compressed size of block 0
    let compressed_size = if num_blocks == 1 {
        // Use compressed_data_size field (header[0x08..0x10])
        let compressed_data_size = u64::from_le_bytes(header[0x08..0x10].try_into().unwrap());
        compressed_data_size as usize
    } else {
        let blk1_ptr = u64::from_le_bytes(header[0x28..0x30].try_into().unwrap());
        let blk1_offset = blk1_ptr & !(1u64 << 63);
        if blk1_offset == 0 {
            return Err(Error::Gcz);
        }
        blk1_offset as usize
    };

    // Skip the remainder of the block pointer + hash tables
    // Reader is currently at byte HEADER_SIZE; data region starts at 0x20 + num_blocks * 12
    let data_start = 0x20u64 + num_blocks as u64 * 12;
    let skip = data_start.saturating_sub(HEADER_SIZE as u64);
    io::copy(&mut reader.take(skip), &mut io::sink())?;

    // Read and decompress block 0
    let mut block_data = vec![0u8; compressed_size];
    reader.read_exact(&mut block_data)?;

    let decompressed = if blk0_ptr & (1u64 << 63) != 0 {
        block_data // Uncompressed
    } else {
        #[cfg(not(feature = "deflate"))]
        return Err(Error::Gcz);

        #[cfg(feature = "deflate")]
        miniz_oxide::inflate::decompress_to_vec_zlib(&block_data)?
    };

    // Copy to header buffer
    let new_header = decompressed
        .first_chunk::<HEADER_SIZE>()
        .ok_or(Error::Gcz)?;
    header.copy_from_slice(new_header);

    Ok(())
}
