use std::{env, process};
use rsgrep::Config;
fn main() {
    let args: Vec<_> = env::args().collect();
    let conf = Config::new(&args).unwrap_or_else(|err| {
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
