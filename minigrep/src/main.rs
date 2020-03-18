use minigrep::Config;
use std::env;
use std::process;
fn main() {
    let cfg = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Arguments error: {:?}", err);
        process::exit(1);
    });
    println!("Query {} from file: {}", cfg.query, cfg.filename);
    minigrep::run(&cfg).expect("io error");
}
