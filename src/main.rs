use clap::Parser;
use cleanup::{Engine, Manager};

fn main() {
    // setup command using clap
    let engine = Engine::parse();

    let mut manager = Manager::new();
    manager.check_error().unwrap_or_else(|err| err.exit());
    // validate user input
    manager.validate(engine).unwrap_or_else(|err| err.exit());
    // execute
    manager.execute().unwrap(); //.unwrap_or_else(|err| err.exit());
}
