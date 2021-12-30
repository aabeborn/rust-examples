use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    let secret = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Please input your guess...");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
    // variable declaration example
    // variable are immutable by default!
    // let apples = 5;
    // to declare a mutable variable
    // let mut bananas = 5;
    // String::new is an associated function. new is a function implemented on the type string
}
