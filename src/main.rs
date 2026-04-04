// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(feature = "cli")]
struct Args {}

#[cfg(feature = "cli")]
fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("help") => {
                println!("Usage: wii-disc-info < FILE");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {})
}

#[cfg(feature = "cli")]
fn main() -> Result<(), lexopt::Error> {
    let _args = parse_args()?;

    let mut reader = std::io::stdin();
    let info = wii_disc_info::query(&mut reader).unwrap();

    println!("Game ID: {}", info.game_id());
    println!("Region: {}", info.region());
    println!("Disc Number: {}", info.disc_number());
    println!("Disc Version: {}", info.disc_version());
    println!("Is Wii: {}", info.is_wii());
    println!("Is GameCube: {}", info.is_gc());
    println!("Game Title: {}", info.game_title());

    Ok(())
}

#[cfg(not(feature = "cli"))]
fn main() {
    println!("Please add the `cli` feature to enable the CLI");
}
