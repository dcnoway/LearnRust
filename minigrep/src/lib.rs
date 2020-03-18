use std::fs;
// use std::io;
use std::error::Error;

pub fn run(cfg: &Config) -> Result<(), Box<dyn Error>> {
    // let mut hit_lines: Vec<&str> = vec![];
    let content = fs::read_to_string(&cfg.filename)?;
    let hit_lines = search(&cfg.query, &content);
    for (line_number,line) in hit_lines.iter().enumerate() {
        println!("{:>6}:{}",line_number,line);
    }
    return Ok(());
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

pub fn search<'a>(query: &str,contents: &'a str) -> Vec<&'a str> {
    let mut result:Vec<&str>=vec![];
    for line in contents.lines() {
        if line.contains(query){
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let content = "Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}
