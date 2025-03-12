use clap::Parser;
use cleanup::Engine;

fn main() -> Result<(), String> {
    println!("In-progress!");

    // setup command using clap
    // initiate manager
    // execute manager

    // TODO: check for the lifetime!!!

    let engine = Engine::parse();
    dbg!(&engine);

    engine.execute()

    // if engine.config.is_some() {
    //     let value = engine.config.unwrap();
    //     println!("Value: {:?}", value);
    // } else {
    //     println!("No value");
    // }
}
