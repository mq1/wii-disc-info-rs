// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(std::io::ErrorKind),

    #[error("Invalid console")]
    InvalidConsole,

    #[error("Invalid game id")]
    InvalidGameId,

    #[error("Invalid game title")]
    InvalidGameTitle,
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err.kind())
    }
}
