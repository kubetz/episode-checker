use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    #[error("error parsing input: {0}")]
    Parse(#[source] nom::Err<()>),

    #[error("wrong JSON data: {0}")]
    WrongJSON(#[source] std::io::Error),

    #[error("cannot load show from the API")]
    CannotLoad(),

    #[error("invalid file {0}")]
    InvalidFile(std::path::PathBuf),

    #[error("episode S{0:0>2}E{1:0>2} not found")]
    NotFound(u16, u8),
}
