use std::env;
use std::fs;
use std::io;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args:\n{:?}", args);
    // let cfg = Config::new(query,filename);
    let cfg = Config::new(&args).unwrap_or_else(|err| {
        println!("Arguments error: {:?}", err);
        process::exit(1);
    });
    println!("Query {} from file: {}", cfg.query, cfg.filename);
    search_file(&cfg).expect("io error");
}

fn search_file(cfg: &Config) -> Result<Vec<usize>, io::Error> {
    let hit_lines: Vec<usize> = vec![];
    let content = fs::read_to_string(&cfg.filename)?;
    println!("File content:\n{}",content);
    return Ok(hit_lines);
}

pub struct Config {
    pub query: String,
    pub filename: String,
    // opt:String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Config {
            query: String::from(&args[1]),
            filename: String::from(&args[2]),
        })
    }
}
