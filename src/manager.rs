use crate::{
    Config, Engine, Kind,
    error::{AppError, AppErrorKind},
};
use serde::Deserialize;
use std::{
    fs,
    path::{self, Path, PathBuf},
};

#[derive(Deserialize, Debug, PartialEq)]
pub struct Manager {
    configs: Vec<Config>,
    dryrun: bool,
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            configs: vec![],
            dryrun: false,
        }
    }

    pub fn validate(&mut self, engine: Engine) -> crate::Result<()> {
        // dryrun
        self.dryrun = engine.dryrun;

        // config
        if let Some(mut path) = engine.config {
            // check relative or absolute path
            path = if path.is_relative() {
                path::absolute(path).unwrap()
            } else {
                path
            };

            // config file exists or not
            if !path.exists() {
                return Err(AppError::new(
                    AppErrorKind::Usage,
                    "config file doesn't exists",
                ));
            }

            // config file is a json file or not?
            let extn = path
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_lowercase();
            if extn != "json" {
                return Err(AppError::new(
                    AppErrorKind::Usage,
                    "config file is not a JSON file, please provide a JSON file",
                ));
            }

            // parse config file
            self.parse(path)?;
            Ok(())
        } else {
            let destination = engine.destination.ok_or(AppError::new(
                AppErrorKind::Usage,
                "Please provide destination",
            ))?;
            let kind = engine
                .kind
                .ok_or(AppError::new(AppErrorKind::Usage, "Please provide kind"))?;

            let patterns = engine.patterns.ok_or(AppError::new(
                AppErrorKind::Usage,
                "Please provide patterns",
            ))?;

            // validate destination path exists or not
            if !destination.exists() {
                return Err(AppError::new(
                    AppErrorKind::Usage,
                    "destination doesn't exists",
                ));
            }

            // make sure destination path is a folder, not file or symlink
            if !destination.is_dir() {
                return Err(AppError::new(
                    AppErrorKind::Usage,
                    "destination is not a directory, please provide directory path as destination!",
                ));
            }

            // format user input
            self.format(destination, kind, patterns)?;
            Ok(())
        }
    }

    pub fn execute(&self) -> crate::Result<()> {
        // loop over each config
        for config in &self.configs {
            helper::remove(
                &config.destination,
                &config.kind,
                &config.patterns,
                self.dryrun,
            );
        }
        Ok(())
    }

    fn add(&mut self, config: Config) {
        self.configs.push(config);
    }

    fn parse(&mut self, path: PathBuf) -> crate::Result<()> {
        let json_data = fs::read_to_string(path)?;
        self.configs = serde_json::from_str(&json_data)?;
        Ok(())
    }

    fn format(
        &mut self,
        destination: PathBuf,
        kind: Kind,
        patterns: Vec<String>,
    ) -> crate::Result<()> {
        self.add(Config::new(destination, kind, patterns));
        Ok(())
    }
}

impl Default for Manager {
    fn default() -> Self {
        Self::new()
    }
}

mod helper {
    use super::*;

    // TODO: think remove need to return Result<...>?
    pub fn remove<P: AsRef<Path>>(destination: P, kind: &Kind, patterns: &[String], dryrun: bool) {
        // pub fn remove(destination: &Path, kind: &Kind, patterns: &[String], dryrun: bool) {
        let destination = destination.as_ref();
        if destination.exists() {
            // get child item of kind
            let children = self::childern(destination);

            // iterate over each child
            for child in &children {
                // if match, then remove
                match self::pattern_check(child, patterns, kind) {
                    Some(_) => {
                        // remove child
                        println!("Removing {:?}...", child);
                        if !dryrun {
                            match self::remove_item(child) {
                                Ok(_) => println!("Removed {:?}...", child),
                                Err(e) => eprintln!("Error: {}", e),
                            }
                        }
                    }
                    None => {
                        if child.is_dir() {
                            self::remove(child, kind, patterns, dryrun);
                        }
                    }
                }
            }
        }
    }

    // TODO: return Result<Vec<PathBuf>, AppError>
    pub fn childern(parent: &Path) -> Vec<PathBuf> {
        let mut children = Vec::new();

        match fs::read_dir(parent) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            children.push(entry.path());

                            // check child is matching with patterns or not
                            // if *kind == Kind::Folder && entry.file_type().unwrap().is_dir() {
                            //     children.push(entry.path());
                            // } else if *kind == Kind::File && entry.file_type().unwrap().is_file() {
                            //     children.push(entry.path());
                            // }
                        }
                        Err(e) => {
                            eprintln!("Error reading directory entry: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading directory: {}", e);
            }
        }

        children
    }

    pub fn pattern_check(path: &Path, patterns: &[String], kind: &Kind) -> Option<usize> {
        // check for folder
        if *kind == Kind::Folder && path.is_dir() {
            let name = path.file_name().unwrap().to_str().unwrap();
            patterns.iter().position(|n| n == name)
        } else if *kind == Kind::File && path.is_file() {
            let extn = path.extension().unwrap().to_str().unwrap();
            patterns.iter().position(|n| n == extn)
        } else {
            None
        }
    }

    pub fn remove_item(path: &Path) -> std::io::Result<()> {
        if path.is_file() {
            fs::remove_file(path)
        } else {
            fs::remove_dir_all(path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_manager() {
        let manager = Manager::new();
        assert_eq!(
            manager,
            Manager {
                configs: vec![],
                dryrun: false
            }
        );
    }

    #[test]
    fn add_config() {
        let mut manager = Manager::new();
        manager.add(Config::new(
            PathBuf::from("/Users/abhinath/productive/pool/Project"),
            Kind::Folder,
            vec![
                String::from("build"),
                String::from("debug"),
                String::from("release"),
            ],
        ));
        assert_eq!(
            manager,
            Manager {
                configs: vec![Config {
                    destination: PathBuf::from("/Users/abhinath/productive/pool/Project"),
                    kind: Kind::Folder,
                    patterns: vec![
                        String::from("build"),
                        String::from("debug"),
                        String::from("release"),
                    ]
                }],
                dryrun: false
            }
        );
    }

    #[test]
    fn check_format() {
        let mut manager = Manager::new();
        manager
            .format(
                PathBuf::from("/Users/abhinath/productive/pool/Project"),
                Kind::Folder,
                vec![
                    String::from("build"),
                    String::from("debug"),
                    String::from("release"),
                ],
            )
            .unwrap();

        assert_eq!(
            manager,
            Manager {
                configs: vec![Config {
                    destination: PathBuf::from("/Users/abhinath/productive/pool/Project"),
                    kind: Kind::Folder,
                    patterns: vec![
                        String::from("build"),
                        String::from("debug"),
                        String::from("release"),
                    ]
                }],
                dryrun: false
            }
        );
    }
}
