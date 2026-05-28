// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{Error, HEADER_SIZE};
use std::io::{self, Read};

pub fn read<R: Read>(reader: &mut R, header: &mut [u8; HEADER_SIZE]) -> Result<(), Error> {
    // Parse metadata
    let num_blocks = u32::from_le_bytes(header[0x1C..0x20].try_into().unwrap());
    let blk0_ptr = u64::from_le_bytes(header[0x20..0x28].try_into().unwrap());
    let blk1_ptr = u64::from_le_bytes(header[0x28..0x30].try_into().unwrap());

    // Calculate compressed size
    let blk0_offset = blk0_ptr & !(1u64 << 63);
    let blk1_offset = blk1_ptr & !(1u64 << 63);
    if blk0_offset >= blk1_offset {
        return Err(Error::GczDecompressionFailed);
    }
    let compressed_size = (blk1_offset - blk0_offset) as usize;

    // Skip to the beginning of block 0
    let skip = (num_blocks as u64 * 12).saturating_sub(HEADER_SIZE as u64 - 0x20);
    io::copy(&mut reader.take(skip), &mut io::sink())?;

    // Read and Decompress
    let mut block_data = vec![0u8; compressed_size];
    reader.read_exact(&mut block_data)?;

    let decompressed = if blk0_ptr & (1u64 << 63) != 0 {
        block_data // Uncompressed
    } else {
        #[cfg(not(feature = "deflate"))]
        return Err(Error::GczDecompressionFailed);

        #[cfg(feature = "deflate")]
        miniz_oxide::inflate::decompress_to_vec_zlib(&block_data)?
    };

    // Copy to Header Buffer
    let new_header = decompressed
        .first_chunk::<HEADER_SIZE>()
        .ok_or(Error::GczDecompressionFailed)?;
    header.copy_from_slice(new_header);

    Ok(())
}
