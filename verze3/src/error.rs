#[derive(Debug)]
pub enum CustomError {
    IO,
    NoGraph,
}

impl From<std::io::Error> for CustomError {
    fn from(value: std::io::Error) -> Self {
        return Self::IO;
    }
}

pub type Result<T> = core::result::Result<T, CustomError>;
