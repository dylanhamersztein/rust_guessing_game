use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=10);

    println!("Enter your guess:");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big! Guess again."),
            Ordering::Less => println!("Too small! Guess again."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
