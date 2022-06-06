use std::{
    error::Error as StdError,
    fmt::Display,
    io,
    num::{ParseFloatError, ParseIntError},
};

use crate::formats;

#[derive(Debug)]
pub enum ColorError {
    ParseHexError(String),
    ParseToIntError(ParseIntError, String),
}

impl Display for ColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ColorError::ParseHexError(ref e) => write!(f, "Failed to parse hex {}", e),
            ColorError::ParseToIntError(ref e, ref input) => {
                write!(f, "Failed to parse string {} into Int. {}", input, e)
            }
        }
    }
}

impl From<ParseIntError> for ColorError {
    fn from(e: ParseIntError) -> Self {
        ColorError::ParseToIntError(e, "".into())
    }
}

impl StdError for ColorError {}

// READER

#[derive(Debug)]
pub enum SchemeReaderError {
    IOError(io::Error, String),
    NoLinesError,
}

impl Display for SchemeReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SchemeReaderError::IOError(ref io_err, ref message) => {
                write!(f, "io::Error occurred: {}. {}", message, io_err)
            }
            SchemeReaderError::NoLinesError => write!(f, "The file read appears to be empty"),
        }
    }
}

impl From<io::Error> for SchemeReaderError {
    fn from(err: io::Error) -> Self {
        SchemeReaderError::IOError(err, "".into())
    }
}

impl StdError for SchemeReaderError {}

// FORMATS

#[derive(Debug)]
pub struct ParseFormatError(pub formats::ColorFormats, pub String);

impl From<ParseFloatError> for ParseFormatError {
    fn from(orig: ParseFloatError) -> Self {
        ParseFormatError(
            formats::ColorFormats::RGBf,
            format!("unable to parse captured string to float: {}", orig),
        )
    }
}
