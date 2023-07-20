use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(query) => query,
            None => return Err("No 'query' specified"),
        };

        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err("No 'filename' specified"),
        };

        Ok(Config {
            query: query,
            filename: filename,
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}
