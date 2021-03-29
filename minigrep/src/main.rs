use {
    minigrep as lib,
    std::{env, error::Error, fs, process},
};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        let query = args.nth(1).ok_or("Query argument missing")?;
        let filename = args.next().ok_or("File argument missing")?;
        let is_case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            is_case_sensitive,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config)
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    lib::search(lib::Config {
        query: &config.query,
        contents: &contents,
        is_case_sensitive: config.is_case_sensitive,
    })
    .iter()
    .for_each(|line| {
        println!("{}", line);
    });

    Ok(())
}
