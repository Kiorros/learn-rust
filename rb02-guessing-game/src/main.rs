use io::stdin;
use Ordering::{Equal, Greater, Less};
use std::cmp::Ordering;
use std::io;
use rand::{Rng, thread_rng};

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut user_input = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        let guess: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input was not a valid number.");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Less => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal => {
                println!("Correct! You win!");
                break;
            }
        }
    }
}
