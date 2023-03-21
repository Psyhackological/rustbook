use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        _ = args.next();

        let query: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = match args.next() {
            Some(arg) => arg.starts_with("-i") || arg.starts_with("--ignore-case"),
            None => env::var("IGNORE_CASE").is_ok(),
        };

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let expected = vec!["safe, fast, productive."];
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let result = search(query, contents);
        assert_eq!(expected, result);
    }

    #[test]
    fn case_insensitive() {
        let expected = vec!["Rust:", "Trust me."];
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        let result = search_case_insensitive(query, contents);
        assert_eq!(expected, result);
    }
    #[test]
    fn flag_true_when_known() {
        let expected = true;
        let args = vec![
            "target/debug/minigrep".to_string(),
            "to".to_string(),
            "poem.txt".to_string(),
            "-i".to_string(),
        ];
        let config = Config::build(args.into_iter());
        let result = config.unwrap().ignore_case; // unwrap from Ok()

        assert_eq!(expected, result);

        let args = vec![
            "target/debug/minigrep".to_string(),
            "to".to_string(),
            "poem.txt".to_string(),
            "--ignore-case".to_string(),
        ];
        let config = Config::build(args.into_iter());
        let result = config.unwrap().ignore_case; // unwrap from Ok()

        assert_eq!(expected, result);
    }
    #[test]
    fn flag_false_when_unknown() {
        let expected = false;
        let args = vec![
            "target/debug/minigrep".to_string(),
            "to".to_string(),
            "poem.txt".to_string(),
            "-a".to_string(),
        ];
        let config = Config::build(args.into_iter());
        let result = config.unwrap().ignore_case; // unwrap from Ok()

        assert_eq!(expected, result);

        let args = vec![
            "target/debug/minigrep".to_string(),
            "to".to_string(),
            "poem.txt".to_string(),
            "--abc".to_string(),
        ];
        let config = Config::build(args.into_iter());
        let result = config.unwrap().ignore_case; // unwrap from Ok()

        assert_eq!(expected, result);
    }
    #[test]
    fn flag_false_when_none() {
        let expected = false;
        let args = vec![
            "target/debug/minigrep".to_string(),
            "to".to_string(),
            "poem.txt".to_string(),
        ];
        let config = Config::build(args.into_iter());
        let result = config.unwrap().ignore_case; // uwwrap from Ok()
        assert_eq!(expected, result);
    }
}
