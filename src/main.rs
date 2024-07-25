#[derive(Debug)]
struct Rectangle {
    width: u16,
    height: u16,
}
impl Rectangle {
    fn square(size: u16) -> Self {
        Self {
            width: size,
            height: size,
        }
       
    }
    fn area(&self)->u16{
        self.width*self.height
    }
}
fn main() {
    let s=Rectangle::square(50);
    let area=Rectangle::area(&s);
    println!("The area is {:?} square pixels",area);
}
