pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(query: String, file_path: String) -> Config {
        Config {
            query,
            file_path,
        }
    }

    pub fn build (args: Vec<String>) -> Result<Config,  &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config::new(args[1].clone(), args[2].clone()))
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
            String::from("Test4"),
        ];

        let config = Config::build(test_args).unwrap();

        assert_eq!(config.query, "Test2");
        assert_eq!(config.file_path, "Test3")
    }
}