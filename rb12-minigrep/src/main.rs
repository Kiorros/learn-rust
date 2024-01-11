use std::{env, process};
use rb12_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ignore_case = env::var("IGNORE_CASE").is_ok();
    let config = Config::build(&args, ignore_case).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = rb12_minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(2);
    }
}
