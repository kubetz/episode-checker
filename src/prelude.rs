use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Error parsing input {0}")]
    Parse(String),

    #[error("Invalid file {0}")]
    InvalidFile(std::path::PathBuf),

    #[error("Cannot load show {0}")]
    CannotLoad(String),

    #[error("Wrong JSON data for {0}")]
    WrongJSON(String),

    #[error("Episode S{1:0>2}E{2:0>2} not found for {0}")]
    NotFound(String, u16, u8),
}
