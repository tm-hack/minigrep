extern crate getopts;

use getopts::Options;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

pub fn parse_config(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
        return Err("not enough arguments");
    }

    let mut opts = Options::new();
    opts.optflag("i", "insensitive", "set insensitive mode");
    opts.optflag("h", "help", "print this help menu");

    let matches = opts.parse(&args[1..]).unwrap();

    let program_name = args[0].clone();
    if matches.opt_present("h") {
        print_usage(&program_name, opts);
        return Err("it's not error. displayed help page.");
    }

    let filename = args[args.len() - 1].clone();
    if filename.starts_with("-") {
        return Err("arguments should be [options] QUERY FILENAME");
    }

    let query = args[args.len() - 2].clone();
    if query.starts_with("-") {
        return Err("arguments should be [options] QUERY FILENAME");
    }

    let case_sensitive = if matches.opt_present("i") {
        false
    } else {
        true
    };

    Ok(Config {
        query,
        filename,
        case_sensitive,
    })
}

fn print_usage(program_name: &str, opts: Options) {
    let brief = format!("Usage: {} [options] QUERY FILENAME", program_name);
    print!("{}", opts.usage(&brief));
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
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
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

/* ----------------------------------------------------------------
    tests for parse_config
---------------------------------------------------------------- */
#[cfg(test)]
mod tests_for_parse_config {
    use super::*;

    #[test]
    fn parse_config_normal_test_1() {
        let command_input = "minigrep -i to poem.txt";
        let args: Vec<String> = command_input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let res = parse_config(&args).unwrap();
        assert_eq!("to", res.query);
        assert_eq!("poem.txt", res.filename);
        assert_eq!("false", res.case_sensitive.to_string());
    }

    #[test]
    fn parse_config_normal_test_2() {
        // case1
        let command_input = "minigrep to poem.txt";
        let args: Vec<String> = command_input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let res = parse_config(&args).unwrap();
        assert_eq!("to", res.query);
        assert_eq!("poem.txt", res.filename);
        assert_eq!("true", res.case_sensitive.to_string());
    }

    #[test]
    fn parse_config_abnormal_test_no_query() {
        let command_input = "minigrep poem.txt";
        let args: Vec<String> = command_input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let res = parse_config(&args).err().unwrap();
        let expect = "not enough arguments";
        assert_eq!(expect, res);
    }

    #[test]
    fn parse_config_abnormal_test_no_filename() {
        let command_input = "minigrep to";
        let args: Vec<String> = command_input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let res = parse_config(&args).err().unwrap();
        let expect = "not enough arguments";
        assert_eq!(expect, res);
    }

    #[test]
    fn parse_config_abnormal_no_args() {
        let command_input = "minigrep";
        let args: Vec<String> = command_input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let res = parse_config(&args).err().unwrap();
        let expect = "not enough arguments";
        assert_eq!(expect, res);
    }

    #[test]
    fn parse_config_abnormal_test_param_error_1() {
        let command_input = "minigrep -i poem.txt";
        let args: Vec<String> = command_input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let res = parse_config(&args).err().unwrap();
        let expect = "arguments shoudl be [options] QUERY FILENAME";
        assert_eq!(expect, res);
    }

    #[test]
    fn parse_config_abnormal_test_param_error_2() {
        let command_input = "minigrep poem.txt -i";
        let args: Vec<String> = command_input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let res = parse_config(&args).err().unwrap();
        let expect = "arguments shoudl be [options] QUERY FILENAME";
        assert_eq!(expect, res);
    }
}

/* ----------------------------------------------------------------
    tests for case_sensitive
---------------------------------------------------------------- */
#[cfg(test)]
mod tests_for_case_sensitive {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
