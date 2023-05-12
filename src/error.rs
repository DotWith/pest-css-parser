use std::num;

use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Int(#[from] num::ParseIntError),
}

pub type Result<T> = std::result::Result<T, Error>;
