use std::env;
use std::fs;
use std::io;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args:\n{:?}", args);
    let (query,filename) = parse_args(&args);
    let cfg = Config::new(query,filename);
    println!("Query {} from file: {}", cfg.query, cfg.filename);
}

fn parse_args(args: &Vec<String>) -> (&str, &str) {
    if args.len() >= 3 {
        return (&args[1], &args[2]);
    } else {
        panic!("Not enought arguments!");
    }
}

fn search_file(cfg:&Config) -> Result<Vec<usize>,io::Error> {
    let mut hit_lines:Vec<usize>=vec![];
    let content = fs::read_to_string(&cfg.filename)?;

    return Ok(hit_lines);
}

pub struct Config{
    pub query:String,
    pub filename:String,
    // opt:String
}

impl Config{
    pub fn new(query:&str,filename:&str) -> Config{
        return Config{query : String::from(query),filename:String::from(filename)};
    }
}