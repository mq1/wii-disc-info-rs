// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{Error, HEADER_SIZE};
use miniz_oxide::inflate::decompress_to_vec_zlib;
use std::io::{self, Read};

fn read_ptr(header: &[u8; HEADER_SIZE], offset: usize) -> u64 {
    u64::from_le_bytes(header[offset..offset + 8].try_into().unwrap())
}

pub fn read<R: Read>(reader: &mut R, header: &mut [u8; HEADER_SIZE]) -> Result<(), Error> {
    let block_size = u32::from_le_bytes(header[0x18..0x1C].try_into().unwrap()) as usize;
    let num_blocks = u32::from_le_bytes(header[0x1C..0x20].try_into().unwrap()) as usize;

    let ptr0 = read_ptr(&header, 0x20);
    let ptr1 = (num_blocks > 1).then(|| read_ptr(&header, 0x28));

    // Skip remaining pointer table entries not yet read + hash table (num_blocks × u32)
    const PTRS_IN_HEADER: usize = (HEADER_SIZE - 0x20) / 8;
    let remaining_ptrs = num_blocks.saturating_sub(PTRS_IN_HEADER);
    let skip = (remaining_ptrs * 8 + num_blocks * 4) as u64;
    io::copy(&mut reader.take(skip), &mut io::sink())?;

    // Now positioned at the start of block data
    let blk0_offset = (ptr0 & !(1u64 << 63)) as usize;
    if blk0_offset > 0 {
        io::copy(&mut reader.take(blk0_offset as u64), &mut io::sink())?;
    }

    let compressed_len = ptr1
        .map(|p| (p & !(1u64 << 63)) as usize - blk0_offset)
        .unwrap_or(block_size);

    let mut block_data = vec![0u8; compressed_len];
    reader.read_exact(&mut block_data)?;

    let decompressed = if ptr0 & (1u64 << 63) != 0 {
        block_data
    } else {
        decompress_to_vec_zlib(&block_data).map_err(|_| Error::GczDecompressionFailed)?
    };

    header.copy_from_slice(
        decompressed
            .get(..HEADER_SIZE)
            .ok_or(Error::GczDecompressionFailed)?,
    );

    Ok(())
}
