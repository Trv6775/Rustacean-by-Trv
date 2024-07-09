fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let mut s = s1 + "-" + &s2 + "-" + &s3;

    println!("{}", s);
    let s4 = &mut s;
    s4.push('!');
    println!("{}", s4);
    let s5=&s4;
    s5.push_str("Wtf");
    println!("{s5}");
}
