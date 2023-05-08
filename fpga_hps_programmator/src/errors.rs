use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum FpgaProgError {
    BadRbfFile,
    IoError(io::Error),
    DeviceTreeCompileError,
    Other(String),
}

impl Error for FpgaProgError {}

impl Display for FpgaProgError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FpgaProgError::BadRbfFile => write!(f, "Oops, rbf file is incorrect"),
            FpgaProgError::IoError(e) => Display::fmt(e, f),
            FpgaProgError::DeviceTreeCompileError => write!(f, "DTC Error"),
            FpgaProgError::Other(err_dscr) => write!(f, "Unknown error: {err_dscr}"),
        }
    }
}

impl From<io::Error> for FpgaProgError {
    fn from(value: io::Error) -> Self {
        FpgaProgError::IoError(value)
    }
}
