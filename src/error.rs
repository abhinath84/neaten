use std::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

use clap::Error as ClapError;

pub struct Error {
    kind: ErrorKind,
    message: String,
}

#[derive(Debug, PartialEq)]
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    Usage,
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
        // format!("[{}]::{}", self.kind.message(), &self.message)
        format!(
            "\u{1b}[1m\u{1b}[31merror:\u{1b}[0m {}!\n\n\u{1b}[1m\u{1b}[4mUsage:\u{1b}[0m \u{1b}[1m{}\u{1b}[0m [OPTIONS]\n\nFor more information, try '\u{1b}[1m--help\u{1b}[0m'.\n",
            &self.message,
            env!("CARGO_PKG_NAME")
        )
        // String::from(
        //     "\u{1b}[1m\u{1b}[31merror:\u{1b}[0m config file doesn't exists!\n\n\u{1b}[1m\u{1b}[4mUsage:\u{1b}[0m \u{1b}[1mcleanup\u{1b}[0m [OPTIONS]\n\nFor more information, try '\u{1b}[1m--help\u{1b}[0m'.\n",
        // )
    }

    fn formatted(&self) -> impl Display {
        if self.kind == ErrorKind::Usage {
            format!(
                "\u{1b}[1m\u{1b}[31merror:\u{1b}[0m {}!\n\n\u{1b}[1m\u{1b}[4mUsage:\u{1b}[0m \u{1b}[1m{}\u{1b}[0m [OPTIONS]\n\nFor more information, try '\u{1b}[1m--help\u{1b}[0m'.\n",
                &self.message,
                env!("CARGO_PKG_NAME")
            )
        } else {
            format!("[{}]::{}", self.kind.message(), &self.message)
        }
    }

    fn write(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.formatted())
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
        Self::new(ErrorKind::Usage, e.to_string())
    }
}

impl ErrorKind {
    pub fn message(&self) -> &str {
        match self {
            Self::Usage => "Usage",
            Self::Internal => "Internal",
        }
    }
}
