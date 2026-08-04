// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#![warn(clippy::all, rust_2018_idioms)]

#[cfg(feature = "cli")]
fn main() {
    futures_executor::block_on(async {
        use std::io::IsTerminal;

        let reader = std::io::stdin();

        if reader.is_terminal() {
            eprintln!("Usage: wii-disc-info < FILE");
            std::process::exit(1);
        }

        let mut reader = futures_util::io::AllowStdIo::new(reader);

        let info = wii_disc_info::Meta::read(&mut reader).await.unwrap();

        println!("Format: {}", info.format());
        println!("Game ID: {}", info.game_id());
        println!("Region: {}", info.region());
        println!("Disc Number: {}", info.disc_number());
        println!("Disc Version: {}", info.disc_version());
        println!("Is Wii: {}", info.is_wii());
        println!("Is GameCube: {}", info.is_gc());
        println!("Game Title: {}", info.game_title());
    })
}

#[cfg(not(feature = "cli"))]
fn main() {
    println!("Please add the `cli` feature to enable the CLI");
}
