use clap::ValueEnum;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug, PartialEq, Clone, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Folder,
    File,
}

impl Default for Kind {
    fn default() -> Self {
        Self::Folder
    }
}

// TODO: try to replace `String` with `&str` (if it's better)
#[derive(Deserialize, Default, Debug, PartialEq)]
pub struct Config {
    pub destination: PathBuf,
    pub kind: Kind,
    pub patterns: Vec<String>,
    pub exclude: Option<Vec<String>>,
}

impl Config {
    pub fn new<P, I, S>(destination: P, kind: Kind, patterns: I, exclude: Option<I>) -> Config
    where
        P: Into<PathBuf>,
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        Config {
            destination: destination.into(),
            kind,
            patterns: patterns.into_iter().map(Into::into).collect(),
            exclude: exclude.map(|e| e.into_iter().map(Into::into).collect()),
        }
    }
}

impl AsRef<Config> for Config {
    fn as_ref(&self) -> &Config {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_config() {
        let config = Config::new(
            "/Users/abhinath/productive/pool/Project",
            Kind::Folder,
            vec!["build", "debug", "release"],
            None,
        );

        assert_eq!(
            config.destination,
            PathBuf::from("/Users/abhinath/productive/pool/Project")
        );

        assert_eq!(
            config,
            Config {
                destination: PathBuf::from("/Users/abhinath/productive/pool/Project"),
                kind: Kind::Folder,
                patterns: vec![
                    String::from("build"),
                    String::from("debug"),
                    String::from("release"),
                ],
                exclude: None,
            }
        );
    }

    #[test]
    fn check_kind() {
        // Folder
        let folder_config = Config::new(
            "/Users/abhinath/productive/pool/Project",
            Kind::Folder,
            vec!["build", "debug", "release"],
            None,
        );
        assert_eq!(folder_config.kind, Kind::Folder);

        // Folder
        let file_config = Config::new(
            "/Users/abhinath/productive/pool/Project",
            Kind::File,
            vec!["build", "debug", "release"],
            None,
        );
        assert_eq!(file_config.kind, Kind::File);
    }

    #[test]
    fn check_lifetime() {
        let destination = "/pool/node";
        let patterns = vec!["dist", "node_modules"];

        let config = Config::new(destination, Kind::Folder, patterns, None);
        assert_eq!(
            config,
            Config {
                destination: PathBuf::from("/pool/node"),
                kind: Kind::Folder,
                patterns: vec![String::from("dist"), String::from("node_modules")],
                exclude: None,
            }
        );

        {
            let inner_destination = "/pool/node";
            let inner_patterns = vec!["dist", "node_modules"];
            let inner_config = Config::new(inner_destination, Kind::Folder, inner_patterns, None);
            assert_eq!(
                inner_config,
                Config {
                    destination: PathBuf::from("/pool/node"),
                    kind: Kind::Folder,
                    patterns: vec![String::from("dist"), String::from("node_modules")],
                    exclude: None,
                }
            );
        }
    }
}
