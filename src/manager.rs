use crate::{
    Config, Engine, Kind,
    error::{Error as SelfError, ErrorKind as SelfErrorKind},
};
use clap::{CommandFactory, Error, error::ErrorKind};
use serde::Deserialize;
use std::{
    fs,
    path::{self, PathBuf},
};

#[derive(Deserialize, Debug, PartialEq)]
pub struct Manager {
    configs: Vec<Config>,
}

impl Manager {
    pub fn new() -> Manager {
        Manager { configs: vec![] }
    }

    pub fn check_error(&self) -> crate::Result<()> {
        Err(SelfError::new(
            SelfErrorKind::Usage,
            String::from("Invalid argument!!!"),
        ))
    }

    pub fn validate(&mut self, engine: Engine) -> Result<(), Error> {
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
                return Err(Engine::command().error(
                    ErrorKind::MissingRequiredArgument,
                    "config file doesn't exists!",
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
                return Err(Engine::command().error(
                    ErrorKind::MissingRequiredArgument,
                    "config file is not a JSON file, please provide a JSON file",
                ));
            }

            // execute cleanup
            self.parse(path)
                .map_err(|msg| Engine::command().error(ErrorKind::MissingRequiredArgument, msg))?;
            Ok(())
        } else {
            let destination = engine.destination.ok_or(Engine::command().error(
                ErrorKind::MissingRequiredArgument,
                "Please provide destination!",
            ))?;

            // validate destination path exists or not
            if !destination.exists() {
                return Err(
                    Engine::command().error(ErrorKind::InvalidValue, "config file doesn't exists!")
                );
            }

            // make sure destination path is a folder, not file or symlink
            if !destination.is_dir() {
                return Err(
                    Engine::command().error(ErrorKind::InvalidValue, "config file doesn't exists!")
                );
            }

            let kind = engine.kind.ok_or(
                Engine::command().error(ErrorKind::MissingRequiredArgument, "Please provide kind!"),
            )?;
            let patterns = engine.patterns.ok_or(Engine::command().error(
                ErrorKind::MissingRequiredArgument,
                "Please provide patterns!",
            ))?;

            // execute cleanup
            self.format(destination, kind, patterns)
                .map_err(|msg| Engine::command().error(ErrorKind::MissingRequiredArgument, msg))?;
            Ok(())
        }
    }

    pub fn execute(&self) -> Result<(), String> {
        // loop over each config
        for config in &self.configs {
            helper::remove(&config.destination, &config.kind, &config.patterns);
        }
        Ok(())
    }

    fn add(&mut self, config: Config) {
        self.configs.push(config);
    }

    fn parse(&mut self, path: PathBuf) -> Result<(), String> {
        // validate the path
        // if relative path then convert to absolute path
        // serialize json file
        // iterate over serialized data and add configs to manager

        let json_data = fs::read_to_string(path).map_err(|err| err.to_string())?;
        self.configs = serde_json::from_str(&json_data).map_err(|err| err.to_string())?;
        Ok(())
    }

    fn format(
        &mut self,
        destination: PathBuf,
        kind: Kind,
        patterns: Vec<String>,
    ) -> Result<(), String> {
        Ok(self.add(Config::new(destination, kind, patterns)))
    }
}

mod helper {
    use super::*;

    pub fn remove(destination: &PathBuf, kind: &Kind, patterns: &Vec<String>) {
        if destination.exists() {
            // get child item of kind
            let children = self::childern(destination);

            // iterate over each child
            for child in &children {
                // println!("Checking {:?}...", child);

                // if match, then remove
                match self::pattern_check(child, patterns, kind) {
                    Some(_) => {
                        // remove child
                        println!("Removing {:?}...", child);

                        // TODO: handle error by logging it on console/log file
                        // self::remove_item(child).unwrap();
                    }
                    None => {
                        if child.is_dir() {
                            self::remove(child, kind, patterns);
                        }
                    }
                }
            }
        }
    }

    pub fn childern(parent: &PathBuf) -> Vec<PathBuf> {
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

    pub fn pattern_check(path: &PathBuf, patterns: &Vec<String>, kind: &Kind) -> Option<usize> {
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

    pub fn remove_item(path: &PathBuf) -> std::io::Result<()> {
        if path.is_file() {
            fs::remove_file(&path)
        } else {
            fs::remove_dir_all(&path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_manager() {
        let manager = Manager::new();
        assert_eq!(manager, Manager { configs: vec![] });
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
                }]
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
                }]
            }
        );
    }
}
