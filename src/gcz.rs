// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BUF_SIZE, HEADER_SIZE, errors::Error};

struct Block0Info {
    compressed_size: usize,
    data_start: usize,
    is_uncompressed: bool,
}

/// Pure helper: Parse and validate GCZ block 0 layout from initial header data
fn parse_block0_info(initial_data: &[u8]) -> Result<Block0Info, Error> {
    let num_blocks = u32::from_le_bytes(initial_data[0x1C..0x20].try_into().unwrap()) as usize;
    if num_blocks == 0 {
        return Err(Error::Gcz);
    }

    // header[0x20..] contains the block pointer table inline
    let blk0_ptr = u64::from_le_bytes(initial_data[0x20..0x28].try_into().unwrap());
    let blk0_offset = blk0_ptr & !(1u64 << 63);
    // Block 0 must be at offset 0 in the data region
    if blk0_offset != 0 {
        return Err(Error::Gcz);
    }

    // Determine compressed size of block 0
    let compressed_size = if num_blocks == 1 {
        // Use compressed_data_size field (header[0x08..0x10])
        let compressed_data_size = u64::from_le_bytes(initial_data[0x08..0x10].try_into().unwrap());
        compressed_data_size as usize
    } else {
        let blk1_ptr = u64::from_le_bytes(initial_data[0x28..0x30].try_into().unwrap());
        let blk1_offset = blk1_ptr & !(1u64 << 63);
        if blk1_offset == 0 {
            return Err(Error::Gcz);
        }
        blk1_offset as usize
    };

    // Check compressed size is reasonable (avoid OOM)
    if compressed_size > 1024 * 1024 {
        return Err(Error::Gcz);
    }

    let data_start = 0x20 + num_blocks * 12;
    let is_uncompressed = (blk0_ptr & (1u64 << 63)) != 0;

    Ok(Block0Info {
        compressed_size,
        data_start,
        is_uncompressed,
    })
}

/// Pure helper: Extract and decompress header from the downloaded block 0 buffer
fn extract_header(buf: &[u8], is_uncompressed: bool) -> Result<[u8; HEADER_SIZE], Error> {
    if is_uncompressed {
        buf.first_chunk::<HEADER_SIZE>().copied().ok_or(Error::Gcz)
    } else {
        #[cfg(not(feature = "deflate"))]
        {
            Err(Error::Gcz)
        }

        #[cfg(feature = "deflate")]
        {
            let mut header = [0u8; HEADER_SIZE];

            let res = miniz_oxide::inflate::decompress_slice_iter_to_slice(
                &mut header,
                std::iter::once(buf),
                true,
                false,
            );

            if let Err(status) = res
                && status != miniz_oxide::inflate::TINFLStatus::HasMoreOutput
            {
                return Err(Error::Gcz);
            }

            Ok(header)
        }
    }
}

/// Synchronous read
pub fn read<R: std::io::Read>(
    reader: &mut R,
    initial_data: &[u8],
) -> Result<[u8; HEADER_SIZE], Error> {
    use std::io::Read;

    let info = parse_block0_info(initial_data)?;
    let mut buf = vec![0u8; info.compressed_size];

    if let Some(overlap) = BUF_SIZE.checked_sub(info.data_start) {
        buf[..overlap].copy_from_slice(&initial_data[info.data_start..]);
        reader.read_exact(&mut buf[overlap..])?;
    } else {
        let skip = (info.data_start - BUF_SIZE) as u64;
        std::io::copy(&mut reader.take(skip), &mut std::io::sink())?;
        reader.read_exact(&mut buf)?;
    }

    extract_header(&buf, info.is_uncompressed)
}

#[cfg(feature = "async")]
/// Asynchronous read
pub async fn read_async<R: futures::AsyncReadExt + Unpin>(
    reader: &mut R,
    initial_data: &[u8],
) -> Result<[u8; HEADER_SIZE], Error> {
    use futures::AsyncReadExt;

    let info = parse_block0_info(initial_data)?;
    let mut buf = vec![0u8; info.compressed_size];

    if let Some(overlap) = BUF_SIZE.checked_sub(info.data_start) {
        buf[..overlap].copy_from_slice(&initial_data[info.data_start..]);
        reader.read_exact(&mut buf[overlap..]).await?;
    } else {
        let skip = (info.data_start - BUF_SIZE) as u64;
        futures::io::copy(&mut reader.take(skip), &mut futures::io::sink()).await?;
        reader.read_exact(&mut buf).await?;
    }

    extract_header(&buf, info.is_uncompressed)
}
