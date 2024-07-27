use rsgrep::*;
use std::env::args;
use std::io::prelude::*;
use std::io::stderr;
use std::process;

fn main() {
    //let argss: Vec<String> = args().collect();
    let mut stderr = stderr();

    let config = Config::new(args()).unwrap_or_else(|err| {
        writeln!(&mut stderr, "error parsing arguments: {}", err)
            .expect("err writing to standard error");
        println!("Problem parsing arguments, {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file : {}", config.filename);
    if let Err(err) = run(config) {
        writeln!(&mut stderr, "error parsing arguments: {}", err)
            .expect("err writing to standard error");
        process::exit(1);
    }
}
