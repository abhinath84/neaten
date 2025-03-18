// https://learning-rust.github.io/docs/custom-error-types/
// https://www.youtube.com/watch?v=KrZ0nmpNVOw&t=1401s

use clap::Error as ClapError;
use serde_json::Error as SerdeJsonError;
use std::{
    backtrace::Backtrace,
    env,
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    io::Error as IoError,
};

fn is_backtrace_enabled() -> bool {
    match env::var("RUST_LIB_BACKTRACE") {
        Ok(value) => value == "1" || value == "full",
        Err(_) => false,
    }
}

pub struct AppError {
    kind: AppErrorKind,
    message: String,
    backtrace: Backtrace,
    // source: Option<Box<dyn Error>>,
}

#[derive(Debug, PartialEq)]
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AppErrorKind {
    Usage,
    Functionality,
    Internal,
}

impl AppError {
    pub fn new(kind: AppErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            backtrace: Backtrace::capture(),
        }
    }

    pub fn exit(&self) -> ! {
        // TODO: replace eprintln!() with user defined/passing Formatter.
        eprintln!("{:?}", self);
        std::process::exit(0);
    }

    // fn message(&self) -> String {
    //     // format!("[{}]::{}", self.kind.message(), &self.message)
    //     format!(
    //         "\u{1b}[1m\u{1b}[31merror:\u{1b}[0m {}!\n\n\u{1b}[1m\u{1b}[4mUsage:\u{1b}[0m \u{1b}[1m{}\u{1b}[0m [OPTIONS]\n\nFor more information, try '\u{1b}[1m--help\u{1b}[0m'.\n",
    //         &self.message,
    //         env!("CARGO_PKG_NAME")
    //     )
    //     // String::from(
    //     //     "\u{1b}[1m\u{1b}[31merror:\u{1b}[0m config file doesn't exists!\n\n\u{1b}[1m\u{1b}[4mUsage:\u{1b}[0m \u{1b}[1mcleanup\u{1b}[0m [OPTIONS]\n\nFor more information, try '\u{1b}[1m--help\u{1b}[0m'.\n",
    //     // )
    // }

    fn formatted(&self) -> String {
        match self.kind {
            AppErrorKind::Usage => self.formatted_usage(),
            AppErrorKind::Functionality => self.formatted_functional(),
            AppErrorKind::Internal => self.formatted_internal(),
        }
    }

    fn formatted_usage(&self) -> String {
        let mut msg = format!("\u{1b}[1m\u{1b}[31merror:\u{1b}[0m {}!\n\n", &self.message);
        msg = format!(
            "{}\u{1b}[1m\u{1b}[4mUsage:\u{1b}[0m \u{1b}[1m{}\u{1b}[0m [OPTIONS]\n\n",
            msg,
            env!("CARGO_PKG_NAME")
        );
        msg = format!(
            "{}For more information, try '\u{1b}[1m--help\u{1b}[0m'.\n",
            msg
        );
        msg
    }

    fn formatted_functional(&self) -> String {
        format!("\n\u{1b}[1m\u{1b}[31merror:\u{1b}[0m {}!\n", &self.message)
    }

    fn formatted_internal(&self) -> String {
        let mut msg = format!("\u{1b}[1m\u{1b}[31merror:\u{1b}[0m {}!\n\n", &self.message);
        msg = format!(
            "{}\u{1b}[1m{}: {}\u{1b}[0m\n\n",
            msg,
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
        if is_backtrace_enabled() {
            msg = format!("{}Callstack:\n{}\n\n", msg, self.backtrace);
            msg = format!(
                "{}please open an issue including this log on \u{1b}[4m{}/issues/new\n",
                msg,
                env!("CARGO_PKG_REPOSITORY")
            );
        } else {
            msg = format!("{}This is an internal error.\n", msg);
            msg = format!(
                "{}You can capture callstack to send us by setting environment variable \u{1b}[4mRUST_LIB_BACKTRACE\u{1b}[0m before re-run {}.\n",
                msg,
                env!("CARGO_PKG_NAME")
            );
            msg = format!("{}Or\n", msg);
            msg = format!(
                "{}Open an issue including the error on \u{1b}[4m{}/issues/new\n",
                msg,
                env!("CARGO_PKG_REPOSITORY")
            );
        }

        msg
    }

    fn write(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.formatted())
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // TODO: replace with message
        self.write(f)
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // TODO: replace with message + debug info
        self.write(f)
        // write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

impl Error for AppError {
    // fn source(&self) -> Option<&(dyn Error + 'static)> {
    //     self.source.as_deref()
    // }
}

impl From<ClapError> for AppError {
    fn from(e: ClapError) -> Self {
        Self::new(AppErrorKind::Usage, e.to_string())
    }
}

impl From<IoError> for AppError {
    fn from(e: IoError) -> Self {
        Self::new(AppErrorKind::Functionality, e.to_string())
    }
}

impl From<SerdeJsonError> for AppError {
    fn from(e: SerdeJsonError) -> Self {
        Self::new(AppErrorKind::Functionality, e.to_string())
    }
}

impl AppErrorKind {
    pub fn message(&self) -> &str {
        match self {
            Self::Usage => "Usage",
            Self::Functionality => "Functionality",
            Self::Internal => "Internal",
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn new() {
//         assert!(true);
//     }
// }
