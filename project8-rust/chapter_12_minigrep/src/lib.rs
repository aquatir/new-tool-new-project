use std::{env, fs};
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {

    /// builds a Config from args
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
        // skip program name parameter at position 0
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string".to_string()),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path".to_string()),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    println!("total matches: {}:", results.len());
    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line)
    //     }
    // }
    // results

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_string().to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_build_arg_correct() {
        let args = [String::from("prog-name"), String::from("search"), String::from("path")];
        let conf = Config::build(&args);

        let conf = conf.unwrap();
        assert_eq!("search", conf.query);
        assert_eq!("path", conf.file_path);
    }

    #[test]
    fn config_build_arg_incorrect_too_few() {
        let args = [String::from("search"), String::from("path")];
        let conf = Config::build(&args);

        let error = conf.err().unwrap();

        let expected_error_text = "Error. Must be invoked with 2 arguments, for example \n cargo run -- searchstring example-filename.txt \nGot 1 arguments instead";
        assert_eq!(expected_error_text, error);
    }

    #[test]
    fn config_build_arg_incorrect_too_many() {
        let args = [String::from("search"), String::from("path"), String::from("arg2"), String::from("arg3")];
        let conf = Config::build(&args);

        let error = conf.err().unwrap();

        let expected_error_text = "Error. Must be invoked with 2 arguments, for example \n cargo run -- searchstring example-filename.txt \nGot 3 arguments instead";
        assert_eq!(expected_error_text, error);
    }

    #[test]
    fn search_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_three_results() {
        let query = "";
        let contents = "\
aaa
aaa
aaa";
        assert_eq!(vec!["aaa", "aaa", "aaa"], search(query, contents));
    }

    #[test]
    fn search_no_result() {
        let query = "aaa";
        let contents = "\
bbb
ccc
ddd
";
        let expected: Vec<String> = vec![];
        assert_eq!(expected, search(query, contents));
    }

    #[test]
    fn search_case_insensitive_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, contents));
    }

    #[test]
    fn search_case_insensitive_three_results() {
        let query = "";
        let contents = "\
aaa
aaa
aaa";
        assert_eq!(vec!["aaa", "aaa", "aaa"], search_case_insensitive(query, contents));
    }

    #[test]
    fn search_case_insensitive_no_result() {
        let query = "aaa";
        let contents = "\
bbb
ccc
ddd
";
        let expected: Vec<String> = vec![];
        assert_eq!(expected, search_case_insensitive(query, contents));
    }

    #[test]
    fn search_case_insensitive_matches_all_cases() {
        let query = "aaa";
        let contents = "\
aaa
AAA
aAa";
        assert_eq!(vec!["aaa", "AAA", "aAa"], search_case_insensitive(query, contents));
    }
}
