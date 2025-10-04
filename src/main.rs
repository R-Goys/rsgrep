use std::{env, process};
use rsgrep::Config;
fn main() {
    let conf = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    match conf.run() {
        Ok(()) => { return; }
        Err(e) => {
            eprintln!("runtime error: {}", e);
            std::process::exit(1);
        }
    }

}
