#[derive(Debug, PartialEq)]
pub enum Kind {
    Folder,
    File,
}

// TODO: try to replace `String` with `&str` (if it's better)
#[derive(Debug, PartialEq)]
pub struct Config<'a> {
    pub destination: &'a str,
    pub kind: Kind,
    pub patterns: Vec<&'a str>,
}

impl<'a> Config<'a> {
    pub fn new(destination: &'a str, kind: Kind, patterns: Vec<&'a str>) -> Config<'a> {
        Config {
            destination,
            kind,
            patterns,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_config() {
        let config = Config::new(
            String::from("/Users/abhinath/productive/pool/Project"),
            Kind::Folder,
            vec!["build", "debug", "release"],
        );

        assert_eq!(
            config,
            Config {
                destination: String::from("/Users/abhinath/productive/pool/Project"),
                kind: Kind::Folder,
                patterns: vec!["build", "debug", "release"]
            }
        );
    }

    #[test]
    fn check_kind() {
        // Folder
        let folder_config = Config::new(
            String::from("/Users/abhinath/productive/pool/Project"),
            Kind::Folder,
            vec!["build", "debug", "release"],
        );
        assert_eq!(folder_config.kind, Kind::Folder);

        // Folder
        let file_config = Config::new(
            String::from("/Users/abhinath/productive/pool/Project"),
            Kind::File,
            vec!["build", "debug", "release"],
        );
        assert_eq!(file_config.kind, Kind::File);
    }
}
