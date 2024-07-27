use std::env::Args;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        //if args.len() < 3 {
        //    return Err("not enough arguments");
        //}
        //let query = &args[1].clone();

        //let filename = &args[2].clone();
        args.next();

        let query = match args.next() {
            Some(s) => s,
            None => return Err("did not get a query"),
        };

        let filename = match args.next() {
            Some(s) => s,
            None => return Err("did not get a filenae"),
        };
        Ok(Config { query, filename })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    let mut content = String::new();

    f.read_to_string(&mut content)?;
    for line in search(&config.query, &content) {
        println!("line: {}", line)
    }
    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    //let mut store = Vec::new();
    //for line in content.lines() {
    //    if line.contains(query) {
    //        store.push(line);
    //    }
    //}
    //store
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let content = "Rust:\nsafe, fast, productive.\npeak three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content))
    }
}
