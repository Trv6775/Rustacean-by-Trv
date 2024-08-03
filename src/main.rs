use std::fs;
use std::io;

fn main() {
    let u = read_username_from_file();
    println!("{:?}", u);
}
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
