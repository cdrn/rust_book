use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // This will print out target/debug/minigrep first, the name of the binary
    // The subsequent elements will be the cli args pased in
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
