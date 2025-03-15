use clap::Parser;
use cleanup::{Engine, Manager};

fn main() {
    // setup command using clap
    let engine = Engine::parse();

    let mut manager = Manager::new();
    manager.check_error().unwrap_or_else(|err| err.exit());
    // validate user input
    manager.validate(engine).unwrap_or_else(|err| {
        // println!("{:#?}", err);
        // println!("\u{1b}[1m\u{1b}[31merror:\u{1b}[0m config file doesn't exists!\n\n\u{1b}[1m\u{1b}[4mUsage:\u{1b}[0m \u{1b}[1mcleanup\u{1b}[0m [OPTIONS]\n\nFor more information, try '\u{1b}[1m--help\u{1b}[0m'.\n");
        println!("{:#?}", err.to_string());
        println!("Application Name: {}", env!("CARGO_PKG_NAME"));

        err.exit()
    });
    // execute
    manager.execute().unwrap(); //.unwrap_or_else(|err| err.exit());
}
