use std::{env, error::Error, fs};
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn build(mut args:impl Iterator<Item=String> ) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next(){
            Some(arg)=>arg,
            None=>return Err("Didn't get a query string"),
        };
        let file_path = match args.next(){
            Some(arg)=>arg,
            None=>return Err("Didn't get a file path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results{
        println!("{:?}",line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "had";
        let contents = "\
I had a dream we were sippin' whiskey neat
Highest floor, The Bowery, nowhere's high enough";
        assert_eq!(
            vec!["I had a dream we were sippin' whiskey neat"],
            search(query, contents)
        );
    }
    #[test]
    fn case_insensitive() {
        let query = "I";
        let contents = "\
I had a dream we were sippin' whiskey neat
Highest floor, The Bowery, nowhere's high enough";
        assert_eq!(
            vec!["I had a dream we were sippin' whiskey neat","Highest floor, The Bowery, nowhere's high enough"],
            search_case_insensitive(query, contents)
        );
    }
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    results
}
