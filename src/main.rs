use my_project::Guess;
use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("Secret number is {secret_number}");
    loop {
        println!("Please input a number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("ENTER A NUMBER :)");
                continue;
            }
        };
        // if guess<1||guess>100{
        //     println!("The secret number will be between 1 and 100");
        //     continue;
        // }
        Guess::new(guess);
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
        println!("You guessed: {guess}");
    }
}
