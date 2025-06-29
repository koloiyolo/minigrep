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

    pub fn build (args: Vec<String>) -> Result<Config,  &'static str> {
        // Base Config constructor
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let mut config = Config::new(args[1].clone(), args[2].clone());

        // Check env for case sensivity
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        if ignore_case {
            config.case = Case::Insensitive;
        }

        // File output
        if args.len() > 3 {
            for i in 2..args.len() - 1 {
                if args[i].contains("-of") || args[i].contains("--output_file") {
                    config.output = Output::File(String::from(&args[i+1]));
                }
            }
        }

        Ok(config)
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

        let config = Config::build(test_args).unwrap();

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

        let config = Config::build(test_args).unwrap();

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

        let config = Config::build(test_args).unwrap();

        assert_eq!(config.query, "Test2");
        assert_eq!(config.file_path, "Test3")
    }
}