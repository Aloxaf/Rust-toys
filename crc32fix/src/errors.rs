use png::DecodingError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ParseError(DecodingError),
    CorrectCrc,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseError(e) => write!(f, "{}", e),
            Error::CorrectCrc => write!(f, "This file has no incorrect crc"),
        }
    }
}
