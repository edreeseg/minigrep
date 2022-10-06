use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let contents = contents.split("\n");
    let mut results: Vec<String> = Vec::new();
    for (i, line) in contents.enumerate() {
        let mut line = line;
        let mut offset = 0;
        while let Some(n) = line.find(&config.query) {
            results.push(format!(
                "({}, {})",
                (i + 1).to_string(),
                (n + 1 + offset).to_string()
            ));
            offset += n + 1;
            line = &line[n + 1..];
        }
    }
    if results.len() > 0 {
        println!(
            "Matches for query found at (line, char): {}",
            results.join(", ")
        );
    } else {
        println!("No matches found for query.");
    }
    Ok(())
}
