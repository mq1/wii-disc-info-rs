// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{BUF_SIZE, HEADER_SIZE, errors::Error};
use futures::{AsyncRead, AsyncReadExt, io};

pub async fn read<R: AsyncRead + Unpin>(
    reader: &mut R,
    initial_data: &[u8],
) -> Result<[u8; HEADER_SIZE], Error> {
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

    // Skip the remainder of the block pointer + hash tables
    // Data region starts at 0x20 + num_blocks * 12
    let data_start = 0x20 + num_blocks * 12;

    // Read and decompress block 0
    let mut buf = vec![0u8; compressed_size].into_boxed_slice();

    if let Some(overlap) = BUF_SIZE.checked_sub(data_start) {
        buf[..overlap].copy_from_slice(&initial_data[data_start..]);
        reader.read_exact(&mut buf[overlap..]).await?;
    } else {
        let skip = (data_start - BUF_SIZE) as u64;
        io::copy(&mut reader.take(skip), &mut io::sink()).await?;
        reader.read_exact(&mut buf).await?;
    }

    if blk0_ptr & (1u64 << 63) != 0 {
        // Uncompressed
        buf.first_chunk::<HEADER_SIZE>().copied().ok_or(Error::Gcz)
    } else {
        #[cfg(not(feature = "deflate"))]
        {
            return Err(Error::Gcz);
        }

        #[cfg(feature = "deflate")]
        {
            let mut header = [0u8; HEADER_SIZE];

            let res = miniz_oxide::inflate::decompress_slice_iter_to_slice(
                &mut header,
                std::iter::once(&buf[..]),
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
