use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        if args.by_ref().count() < 3 {
            return Err("Not enough arguments.");
        }

        let Some(query) = args.next() else {
            return Err("No query");
        };

        let Some(file_path) = args.next() else {
            return Err("No file path");
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    if !config.ignore_case {
        for line in search(&config.query, &contents) {
            println!("{}", line);
        }
    } else {
        for line in case_insensitive_search(&config.query, &contents) {
            println!("{}", line);
        }
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //let results:Vec<&str> = contents.split('\n');
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for lines in contents.lines() {
        if lines.to_lowercase().contains(&query) {
            results.push(lines.trim());
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Trust me";
        assert_eq!(vec!["Rust:", "Trust me"], case_insensitive_search(query, contents)); }

}
