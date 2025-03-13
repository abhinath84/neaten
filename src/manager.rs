use crate::{Config, Engine, Kind};
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

    pub fn validate(&mut self, engine: Engine) -> Result<(), Error> {
        // config
        if let Some(mut path) = engine.config {
            // check relative or absolute path
            path = if path.is_relative() {
                path::absolute(path).unwrap()
            } else {
                path
            };
            // println!("{:?}", path);

            // execute cleanup
            self.parse(path)
                .map_err(|msg| Engine::command().error(ErrorKind::MissingRequiredArgument, msg))?;
            println!("{:#?}", self);
            Ok(())
        } else {
            let destination = engine.destination.ok_or(Engine::command().error(
                ErrorKind::MissingRequiredArgument,
                "Please provide destination!",
            ))?;
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
            println!("{:#?}", self);
            Ok(())
        }
    }

    pub fn execute(&self) -> Result<(), String> {
        // loop over each config
        for config in &self.configs {
            Self::remove(
                config.destination.clone(),
                config.kind.clone(),
                config.patterns.clone(),
            );
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

    fn remove(destination: PathBuf, kind: Kind, patterns: Vec<String>) {
        // get child item of kind
        // iterate over each child
        // check child is matching with patterns or not
        // if match, then remove
        // if not, then call remove() with child
        unimplemented!()
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
