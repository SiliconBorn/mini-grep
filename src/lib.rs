use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("NOT ENOUGH ARGUMENTS");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let contents =
    //     fs::read_to_string(config.filename).expect("SOMETHING WENT WRONG WHILE READINFG THE FILE");

    let contents = fs::read_to_string(config.filename)?;

    println!("CONTENTS OF THE FILE: \n{}", contents);
    Ok(())
}
