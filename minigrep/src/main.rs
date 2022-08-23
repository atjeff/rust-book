use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    search(config)
}

fn search(config: Config) {
    let contents =
        fs::read_to_string(config.file_path).expect("Expected to be able to read the file");

    let location_of_query = contents.find(&config.query).unwrap();

    println!(
        "Found at: {}\n{}",
        location_of_query,
        &contents[location_of_query..location_of_query + config.query.len()]
    )
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from("Not enough arguments"));
        }

        let query = args[1].to_string();
        let file_path = args[2].to_string();

        Ok(Config { query, file_path })
    }
}
