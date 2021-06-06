use std::fs;
use std::error::Error;
use colored::Colorize;
use std::env;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    //println!("Content of file: \n{}", content);
    /*
    let results = if config.case_sensitive {
        search(&config.query, &content);
    } else {
        search_case_insensitive(&config.query, &content);
    };
    */
    let results = search_case_insensitive(&config.query, &content);

    for line in results {
        println!("{}", line.green());
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        return Ok(Config { query, filename, case_sensitive });
    }
}

pub fn search<'a>(query: &'a str, content: &'a str) ->  Vec<&'a str> {

    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {

    let mut result = Vec::new();
    let query = query.to_uppercase();
    for line in content.lines() {
        if line.to_uppercase().contains(&query) {
            result.push(line);
        }
    }

    result

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
save, fast, productive.
Pick three.";

        assert_eq!(vec!["save, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
save, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}