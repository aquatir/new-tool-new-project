use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() != 3 {
            return Err(format!(
                "Error. Must be invoked with 2 arguments, for example \n cargo run -- searchstring example-filename.txt \nGot {} arguments instead",
                args.len() - 1
            ));
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let mut matches = 0;
    for line in search(&config.query, &contents) {
        println!("{line}");
        matches += 1;
    }
    println!("total matches: {}", matches);

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = &query.to_string().to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
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


}
