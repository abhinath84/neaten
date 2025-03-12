use crate::{Config, Kind};
// use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct Manager<'a> {
    configs: Vec<Config<'a>>,
}

impl<'a> Manager<'a> {
    pub fn new() -> Manager<'a> {
        Manager { configs: vec![] }
    }

    fn add(&mut self, config: Config<'a>) {
        self.configs.push(config);
    }

    pub fn parse(&self, path: &str) -> Result<(), String> {
        // validate the path
        // if relative path then convert to absolute path
        // serialize json file
        // iterate over serialized data and add configs to manager
        unimplemented!()
    }

    pub fn format(
        &mut self,
        destination: &'a str,
        kind: Kind,
        patterns: Vec<&'a str>,
    ) -> Result<(), String> {
        Ok(self.add(Config::new(destination, kind, patterns)))
    }

    pub fn execute(&self) -> Result<(), String> {
        // Ok(())
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
            "/Users/abhinath/productive/pool/Project",
            Kind::Folder,
            vec!["build", "debug", "release"],
        ));
        assert_eq!(
            manager,
            Manager {
                configs: vec![Config {
                    destination: "/Users/abhinath/productive/pool/Project",
                    kind: Kind::Folder,
                    patterns: vec!["build", "debug", "release"]
                }]
            }
        );
    }

    #[test]
    fn check_format() {
        let mut manager = Manager::new();
        manager
            .format(
                "/Users/abhinath/productive/pool/Project",
                Kind::Folder,
                vec!["build", "debug", "release"],
            )
            .unwrap();

        assert_eq!(
            manager,
            Manager {
                configs: vec![Config {
                    destination: "/Users/abhinath/productive/pool/Project",
                    kind: Kind::Folder,
                    patterns: vec!["build", "debug", "release"]
                }]
            }
        );
    }
}
