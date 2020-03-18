use std::env;
use std::process;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args:\n{:?}", args);
    // let cfg = Config::new(query,filename);
    let cfg = Config::new(&args).unwrap_or_else(|err| {
        println!("Arguments error: {:?}", err);
        process::exit(1);
    });
    println!("Query {} from file: {}", cfg.query, cfg.filename);
    minigrep::search_file(&cfg).expect("io error");
}
