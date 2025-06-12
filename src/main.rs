use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args = env::args();

    let config = match Config::build(args) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error parsing arguments: {e}");
            process::exit(1);
        }
    };

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
