use super::{Config, Kind, Manager};
use clap::{ArgAction, Parser};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Engine {
    #[arg(long, short)]
    pub config: Option<PathBuf>,

    /// destination directory path(either absolute or relative path).
    #[arg(long, short)]
    pub destination: Option<PathBuf>,

    /// what kind of item wants to remove.
    #[arg(long, short, value_enum)]
    pub kind: Option<Kind>,

    /// List of patterns to remove.
    #[arg(long, short, action = ArgAction::Append, value_delimiter = ',')]
    pub patterns: Option<Vec<String>>,
}

impl Engine {
    pub fn execute(self) -> Result<(), String> {
        // config
        if let Some(path) = self.config {
            let manager = Manager::new();
            manager.parse(path.to_str().unwrap()).unwrap();
            manager.execute();
        } else {
            // do stuff for standard input
            let destination = self.destination.unwrap();
            let kind = self.kind.unwrap();
            let patterns = self.patterns.unwrap().into_iter().map(|x| x.as_str()).collect::Vec<&str>();

            let mut manager = Manager::new();
            manager.format(destination.to_str().unwrap(), kind, patterns);
            manager.execute();
        }
        Ok(())
    }
}
