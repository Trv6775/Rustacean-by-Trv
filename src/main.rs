use std::{env,process};
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        // println!("proble parsing arguments: {}", err);
        process::exit(1);
    });
    println!("search for {}", config.query);
    println!("In file {}", config.file_name);
    if let Err(e)= minigrep::run(config){
        // println!("Application error: {e}");
        process::exit(1);
    };
}
