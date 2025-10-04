use std::{error::Error, fs, env};

pub struct Config {
    pub query:          String,
    pub file_path:      String,
    pub ignore_case:    bool,
}

impl Config {
    pub fn new(args: &[String])->Result<Config, String> {
        if args.len() < 3 {
            return Err(format!("Expect command: \n[program] [query] [file_path] \nbut get \n{:?}", args));
        }
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: ignore_case,
        })
    }
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(&self.file_path)?;
        
        let ans = match self.ignore_case {
            true => search_case_insensitive(&self.query, &contents),
            _ => search(&self.query, &contents)
        };

        for line in ans {
            println!("{line}")
        }
        Ok(())
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new(); 
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn two_result() {
        let query = "rUSt";
        let content = "\
Rust:
safe, fast, productive.
pick three.";
        assert_eq!(vec!["Rust:"], search_case_insensitive(query, content));
    }
}