//! # minigrep
//! minigrep is a tool write by rust beginner's excercise code.
//!  
//! It chould search the given file with given query string, 
//! and print out all the line number and line content which mach the query string.
//! 
//! Usage:
//! ```
//! minigrep <query string> <file path>
//! ```
use std::fs;
// use std::io;
use std::error::Error;

/// search func
pub fn run(cfg: &Config) -> Result<(), Box<dyn Error>> {
    // let mut hit_lines: Vec<&str> = vec![];
    let content = fs::read_to_string(&cfg.filename)?;
    let hit_lines = search(&cfg.query, &content);
    for (line_number, line) in hit_lines.iter() {
        println!("{:>6}:{}", line_number, line);
    }
    return Ok(());
}

/// parsing and holding command line args
/// # Example
/// 
/// ```
/// use std::env;
/// let cfg = Config::new(env::args()).unwrap_or_else(|err| {
///     eprintln!("Arguments error: {:?}", err);
///     process::exit(1);
/// });
/// ```
pub struct Config {
    pub query: String,
    pub filename: String,
    // opt:String
}

impl Config {
    /// Construct function, parsing command line args
    /// # Example
    /// ```
    /// let cfg = Config::new(std::env::args()).unwrap();
    /// ```
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); //skip the first arg that is the fullpath of the binary file
        let query: String;
        let filename: String;
        if let Some(s) = args.next() {
            query = s;
        } else {
            return Err("Missing query parten!");
        }
        if let Some(s) = args.next() {
            filename = s;
        } else {
            return Err("Missing file name!");
        }
        // if args.len() < 3 {
        //     return Err("Not enough arguments");
        // }
        Ok(Config { query, filename })
    }
}

/// search query string in contents, return a vec holding (line number,line content)
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    // let mut result: Vec<(usize, &'a str)> = vec![];
    // for (i, line) in contents.lines().enumerate() {
    //     if line.contains(query) {
    //         result.push((i + 1, line));
    //     }
    // }
    // result
    contents.lines()
        .enumerate()
        .filter(|(_,line)| line.contains(query))
        .map(|(idx,line)|(idx+1,line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let content = "Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!("safe, fast, productive.", search(query, content)[0].1);
        assert_eq!(1, search(query, content).len());
    }
}
