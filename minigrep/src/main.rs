use minigrep;
use std::{env, error::Error, fs, process};

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub is_case_sensitive: bool,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = &args[1];
        let filename = &args[2];
        let is_case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            is_case_sensitive,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in minigrep::search(minigrep::Config {
        query: &config.query,
        contents: &contents,
        is_case_sensitive: config.is_case_sensitive,
    }) {
        println!("{}", line);
    }

    Ok(())
}
