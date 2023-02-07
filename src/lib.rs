use std::{fs, error, env};

pub fn run(cfg: Config) -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(cfg.file_path)?;
    let result = if cfg.case_insensitive { search_case_insensitive(&cfg.query , &content) } else { search(&cfg.query, &content) };
    for line in result {
        println!("{line}");
    }
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
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 { return Err("Not enough arguments passed") }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let case_insensitive: bool = if let Some(arg) = args.get(3) {
            arg == CASE_INSENSITIVE_ARG
        } else {
            env::var(INGORE_CASE_ENV).is_ok()
        };

        Ok(Config { query, file_path, case_insensitive })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut lines = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            lines.push(line);
        }
    }
    lines
} 

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut lines = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            lines.push(line);
        }
    }
    lines
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