use std::{env, process};
use fuctiona_language::Config;

fn main(){
    // let args:Vec<String>=env::args().collect();
    let config=Config::build(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e)=fuctiona_language::run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
