use std::{fs, error, env};

pub fn run(cfg: Config) -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(cfg.file_path)?;
    let result = if cfg.case_insensitive { search_case_insensitive(&cfg.query , &content) } else { search(&cfg.query, &content) };
    result.iter().for_each(|e| println!("{e}"));
    Ok(())
}

const CASE_INSENSITIVE_ARG: &str = "--ignore-case";
const INGORE_CASE_ENV: &str = "IGNORE_CASE";

pub struct Config {
    query: String,
    file_path: String,
    case_insensitive: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Skip the unused first argument
        args.next();

        let query = match args.next() {
            Some(v) => v,
            None => return Err("Empty argument for a query")
        };

        let file_path = match args.next() {
            Some(v) => v,
            None => return Err("Empty argument for a path")
        };

        let case_insensitive: bool = if let Some(arg) = args.next() {
            arg == CASE_INSENSITIVE_ARG
        } else {
            env::var(INGORE_CASE_ENV).is_ok()
        };

        Ok(Config { query, file_path, case_insensitive })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
} 

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_test() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_case_insensitive_test() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}