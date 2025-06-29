use std::fs;
use std::error::Error;

pub mod config;
use config::Config;

pub fn run (config: Config ) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &file_contents) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
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
}