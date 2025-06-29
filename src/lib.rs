use std::fs;
use std::error::Error;

pub mod config;
use config::Config;

pub fn run (config: Config ) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(&config.file_path)?;

    let result = search(&config.query, &file_contents);

    config.output(result);

    Ok(())
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) || query == "_"{
            results.push(line);
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
    fn multiple_results() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
ductape";

        assert_eq!(vec!["safe, fast, productive.", "ductape"], search(query, contents));
    }

    #[test]
    fn catch_all() {
        let query = "_";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
ductape";

        assert_eq!(vec![ "Rust:",
                         "safe, fast, productive.", 
                         "Pick three.",
                         "ductape"
                        ], search(query, contents));
    }
    #[test]
    fn catch_none() {
        let query = "sdasdasd";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
ductape";

        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }
}