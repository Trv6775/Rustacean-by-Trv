// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };
//     let rect3 = Rectangle {
//         width: 60,
//         height: 40,
//     };
//     println!("Can rect1 hold rect2 ?{}", rect1.can_hold(&rect2),);
//     println!("Can rect1 hold rect3 ?{}", rect1.can_hold(&rect3),);
//     println!("The area of the rectangle is {} square pixels",rect1.area(),);
//     println!("The area of the rectangle is {} square pixels",rect2.area(),);
//     println!("The area of the rectangle is {} square pixels",rect3.area(),);
// }
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

fn value_in_cents(coin:Coin)->u8{
    match coin{
        Coin::Penny=>1,
        Coin::Nickel=>5,
        Coin::Dime=>10,
        Coin::Quater(state)=>{
            println!("State quater from {:?}!",state);
            25
        }
    }
}
fn main(){
    let type_of_quater_be=value_in_cents(Coin::Quater(UsState::Alaska));
    println!("{}",type_of_quater_be);
}
