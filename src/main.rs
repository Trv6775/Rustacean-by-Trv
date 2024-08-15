use std::fmt::Display;
fn longest_with_announcement<'a,T>(
    x:&'a str,
    y:&'a str,
    ann:T
)->&'a str where T:Display{
    println!("Announcement! {ann}");
    if x.len()>y.len(){
        x
    }else {
        y
    }
}
fn main() {
    let s1="abcd";
    let s2="abc";
    let longest=longest_with_announcement(&s1, &s2, "It is");
    println!("The longest string is {longest}");
}
