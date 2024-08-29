use std::collections::HashMap;

fn main() {
    let five=Some(5);
    let six=plus_one(five);
    let none=plus_one(None);
    println!("{:?} {:?} {:?}",five,six,none);
}
fn plus_one(a: Option<i32>) -> Option<i32> {
    match a {
        Some(i) => Some(i + 1),
        None => None,
    }
}
