use std::{num::ParseIntError, str::Utf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("`{0}`")]
    Generic(String),

    #[error("`{0}`")]
    ParseInt(#[from] ParseIntError),

    #[error("`{0}`")]
    Utf8(#[from] Utf8Error),

    #[error("io error:{0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
