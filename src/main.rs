// use std::collections::HashMap;

// fn main() {
//     let field_name = String::from("Favorite Color");
//     let field_value = String::from("Blue");
//     let mut map=HashMap::new();
//     let m=map.insert(field_name, field_value);
//     println!("{:?}{} {}",m)
// }

use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    ///The hashmap has been overwritten by 
    /// the below hashmap
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}",scores);
}
