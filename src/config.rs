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