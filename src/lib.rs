use std::{error::Error, fs, env};

pub struct Config {
    pub query:          String,
    pub file_path:      String,
    pub ignore_case:    bool,
}

impl Config {
    pub fn new(mut args:impl Iterator<Item = String>)->Result<Config, String> {
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        args.next();
        let query = match args.next() {
            Some(arg ) => arg,
            None => return Err(format!("Didn't get query param")),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err(format!("Didn't get file path param")),
        };
        Ok(Config {
            query: query,
            file_path: file_path,
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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()    

}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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