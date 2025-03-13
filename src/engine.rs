use super::Kind;
use clap::{ArgAction, Parser};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Engine {
    /// config file path(either absolute or relative path).
    #[arg(long, short)]
    pub config: Option<PathBuf>,

    /// destination directory path(either absolute or relative path).
    #[arg(long, short)]
    pub destination: Option<PathBuf>,

    /// what kind of item wants to remove.
    #[arg(long, short, value_enum)]
    pub kind: Option<Kind>,

    /// List of patterns to remove(comma separated value).
    #[arg(long, short, action = ArgAction::Append, value_delimiter = ',')]
    pub patterns: Option<Vec<String>>,
}
