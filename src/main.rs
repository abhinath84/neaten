use clap::Parser;
use neaten::{Engine, Manager};

fn main() {
    // setup command using clap
    let engine = Engine::parse();

    let mut manager = Manager::new();
    // validate user input
    manager.validate(engine).unwrap_or_else(|err| err.exit());
    // execute
    manager.execute().unwrap(); //.unwrap_or_else(|err| err.exit());
}
