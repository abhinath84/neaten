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

    /// dry-run to check removable item list.
    #[arg(long)]
    pub dryrun: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use std::path::PathBuf;

    #[test]
    fn config_long() {
        let args = vec![
            "neaten",
            "--config",
            "/Users/abhinath/productive/pool/Project/neaten/sample/config.json",
        ];
        let result = Engine::try_parse_from(args);
        assert!(result.is_ok());
        let config = result.unwrap().config.unwrap();
        assert_eq!(
            config,
            PathBuf::from("/Users/abhinath/productive/pool/Project/neaten/sample/config.json")
        );
    }

    #[test]
    fn config_short() {
        let args = vec![
            "neaten",
            "-c",
            "/Users/abhinath/productive/pool/Project/neaten/sample/config.json",
        ];
        let result = Engine::try_parse_from(args);
        assert!(result.is_ok());
        let config = result.unwrap().config.unwrap();
        assert_eq!(
            config,
            PathBuf::from("/Users/abhinath/productive/pool/Project/neaten/sample/config.json")
        );
    }

    #[test]
    fn missing_options() {
        let args = vec!["neaten"];
        let result = Engine::try_parse_from(args);
        assert!(result.is_ok());
        let engine = result.unwrap();
        assert!(engine.config.is_none());
    }

    #[test]
    fn destination_and_others_long() {
        let args = vec![
            "neaten",
            "--destination",
            "/Users/abhinath/productive/pool/Project/neaten/sample/config.json",
            "--kind",
            "folder",
            "--patterns",
            "dist,node_modules",
        ];
        let result = Engine::try_parse_from(args);
        assert!(result.is_ok());
        let engine = result.unwrap();
        assert_eq!(
            engine.destination.unwrap(),
            PathBuf::from("/Users/abhinath/productive/pool/Project/neaten/sample/config.json")
        );
        assert_eq!(engine.kind.unwrap(), super::super::Kind::Folder);
        assert_eq!(
            engine.patterns.unwrap(),
            vec![String::from("dist"), String::from("node_modules")]
        );
    }

    #[test]
    fn destination_and_others_short() {
        let args = vec![
            "neaten",
            "-d",
            "/Users/abhinath/productive/pool/Project/neaten/sample/config.json",
            "-k",
            "folder",
            "-p",
            "dist,node_modules",
        ];
        let result = Engine::try_parse_from(args);
        assert!(result.is_ok());
        let engine = result.unwrap();
        assert_eq!(
            engine.destination.unwrap(),
            PathBuf::from("/Users/abhinath/productive/pool/Project/neaten/sample/config.json")
        );
        assert_eq!(engine.kind.unwrap(), crate::Kind::Folder);
        assert_eq!(
            engine.patterns.unwrap(),
            vec![String::from("dist"), String::from("node_modules")]
        );
    }

    #[test]
    fn missing_destination() {
        let args = vec![
            "neaten",
            "--kind",
            "folder",
            "--patterns",
            "dist,node_modules",
        ];
        let result = Engine::try_parse_from(args);
        assert!(result.is_ok());
        let engine = result.unwrap();
        assert!(engine.destination.is_none());
    }

    #[test]
    fn missing_kind() {
        let args = vec![
            "neaten",
            "--destination",
            "/Users/abhinath/productive/pool/Project/neaten/sample/config.json",
            "--patterns",
            "dist,node_modules",
        ];
        let result = Engine::try_parse_from(args);
        assert!(result.is_ok());
        let engine = result.unwrap();
        assert!(engine.kind.is_none());
    }

    #[test]
    fn missing_patterns() {
        let args = vec![
            "neaten",
            "--destination",
            "/Users/abhinath/productive/pool/Project/neaten/sample/config.json",
            "--kind",
            "folder",
        ];
        let result = Engine::try_parse_from(args);
        assert!(result.is_ok());
        let engine = result.unwrap();
        assert!(engine.patterns.is_none());
    }

    #[test]
    fn dryrun() {
        let args = vec![
            "neaten",
            "--config",
            "/Users/abhinath/productive/pool/Project/neaten/sample/config.json",
            "--dryrun",
        ];
        let result = Engine::try_parse_from(args);
        assert!(result.is_ok());
        let engine = result.unwrap();
        assert_eq!(
            engine.config.unwrap(),
            PathBuf::from("/Users/abhinath/productive/pool/Project/neaten/sample/config.json")
        );
        assert!(engine.dryrun);
    }
}
