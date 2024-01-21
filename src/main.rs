use minigrep::run;
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("PROBLEM PARSING THE ARGUMENTS: {}", err);
        process::exit(1)
    });
    // println!("{:?}", args);
    println!("SEARCHING FOR: {}", config.query);
    println!("IN FILE: {}", config.filename);

    if let Err(e) = run(config) {
        println!("APLLICATION ERROR: {}", e);
        process::exit(1);
    }
}
