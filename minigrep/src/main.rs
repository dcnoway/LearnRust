use minigrep::Config;
use std::env;
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
    minigrep::run(&cfg).expect("io error");
}
