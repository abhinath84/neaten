use std::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

use clap::Error as ClapError;

pub struct Error {
    kind: ErrorKind,
    message: String,
}

#[derive(Debug)]
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    Validation,
    Internal,
}

impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    pub fn exit(&self) -> ! {
        eprintln!("{:?}", self);
        std::process::exit(0);
    }

    fn message(&self) -> String {
        format!("[{}]::{}", self.kind.message(), &self.message)
    }

    fn write(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.write(f)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.write(f)
    }
}

impl StdError for Error {}

impl From<ClapError> for Error {
    fn from(e: ClapError) -> Self {
        Self::new(ErrorKind::Validation, e.to_string())
    }
}

impl ErrorKind {
    pub fn message(&self) -> &str {
        match self {
            Self::Validation => "Validation",
            Self::Internal => "Internal",
        }
    }
}
