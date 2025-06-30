use std::{env, fs, io::Write};

pub enum Output {
    File(String),
    Cli,
}

pub enum Case {
    Sensitive,
    Insensitive,
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case: Case,
    output: Output,
}

impl Config {
    pub fn new(query: String, file_path: String) -> Config {
        Config {
            query,
            file_path,
            output: Output::Cli,
            case: Case::Sensitive,
        }
    }

    pub fn build (
            mut args: impl Iterator<Item = String>
        ) -> Result<Config, String> {
        
        args.next();
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string".to_string()),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err(("Didn't get a file_path string").to_string()),
        };

        let mut config = Config::new(
            query,
            file_path
        );

        // Check env for case sensivity
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        if ignore_case {
            config.case = Case::Insensitive;
        }

        // File output
        if let Some(arg) = args.next() {
            if arg == "-of".to_string() || arg == "--output_file"  {
                if let Some(path) = args.next() {
                    config.output = Output::File(path);
                } 
            }
        }

        Result::Ok(config)
    }

    pub fn output(&self, content: Vec<&str>) {
        match &self.output {
            Output::Cli => {
                for line in content {
                    println!("{line}");
                }
            },
            Output::File(path) => {
                match fs::File::create(path) {
                    Ok(mut file) => {
                        for line in content {
                            if let Err(e) = writeln!(file, "{}", line) {
                                eprintln!("Failed to write line: {e}");
                            }
                            println!("{line}");
                        }
                    },
                    Err(e) => panic!("Failed to create file: {e}"),
                }
            },
        
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_method() {
        let test_args = vec![
            String::from("Test1"),
            String::from("Test2"),
            String::from("Test3"),
        ];

        let config = Config::build(
            test_args.into_iter()
        ).unwrap();

        assert_eq!(config.query, "Test2");
        assert_eq!(config.file_path, "Test3");
    }

    #[test]
    fn test_build_method_with_extra_arg() {
        let test_args = vec![
            String::from("Test1"),
            String::from("Test2"),
            String::from("Test3"),
            String::from("Test4"),
        ];

        let config = Config::build(
            test_args.into_iter()
        ).unwrap();
        
        assert_eq!(config.query, "Test2");
        assert_eq!(config.file_path, "Test3")
    }

    #[test]
    fn test_build_method_of_no_file_specified() {
        let test_args = vec![
            String::from("Test1"),
            String::from("Test2"),
            String::from("Test3"),
            String::from("-of"),
        ];

        let config = Config::build(
            test_args.into_iter()
        ).unwrap();

        assert_eq!(config.query, "Test2");
        assert_eq!(config.file_path, "Test3")
    }
}