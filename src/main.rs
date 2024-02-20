use minigrep_rust::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("{} and file name is {}", config.query, config.file_name);
    if let Err(e) = minigrep_rust::run(config) {
        eprint!("Application error: {}", e);
        process::exit(1);
    }
}
