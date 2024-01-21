use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("NOT ENOUGH ARGUMENTS");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let contents =
    //     fs::read_to_string(config.filename).expect("SOMETHING WENT WRONG WHILE READINFG THE FILE");

    let contents = fs::read_to_string(config.filename)?;

    println!("CONTENTS OF THE FILE: \n{}", contents);
    Ok(())
}

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
