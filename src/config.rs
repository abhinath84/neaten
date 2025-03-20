use clap::ValueEnum;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug, PartialEq, Clone, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Folder,
    File,
}

// TODO: try to replace `String` with `&str` (if it's better)
#[derive(Deserialize, Debug, PartialEq)]
pub struct Config {
    pub destination: PathBuf,
    pub kind: Kind,
    pub patterns: Vec<String>,
    pub exclude: Option<Vec<String>>,
}

impl Config {
    pub fn new(
        destination: PathBuf,
        kind: Kind,
        patterns: Vec<String>,
        exclude: Option<Vec<String>>,
    ) -> Config {
        Config {
            destination,
            kind,
            patterns,
            exclude,
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
            PathBuf::from("/Users/abhinath/productive/pool/Project"),
            Kind::Folder,
            vec![
                String::from("build"),
                String::from("debug"),
                String::from("release"),
            ],
            None,
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
            PathBuf::from("/Users/abhinath/productive/pool/Project"),
            Kind::Folder,
            vec![
                String::from("build"),
                String::from("debug"),
                String::from("release"),
            ],
            None,
        );
        assert_eq!(folder_config.kind, Kind::Folder);

        // Folder
        let file_config = Config::new(
            PathBuf::from("/Users/abhinath/productive/pool/Project"),
            Kind::File,
            vec![
                String::from("build"),
                String::from("debug"),
                String::from("release"),
            ],
            None,
        );
        assert_eq!(file_config.kind, Kind::File);
    }

    #[test]
    fn check_lifetime() {
        let destination = PathBuf::from("/pool/node");
        let patterns = vec![String::from("dist"), String::from("node_modules")];

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
            let inner_destination = PathBuf::from("/pool/node");
            let inner_patterns = vec![String::from("dist"), String::from("node_modules")];
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
