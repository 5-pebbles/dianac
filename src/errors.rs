use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    IoError(IoError),
}

impl From<IoError> for Error {
    fn from(value: IoError) -> Self {
        Self::IoError(value)
    }
}
