extern crate rand;

use rand::*;
use std::cmp::Ordering;
use std::io;
use std::io::Error;

fn main() {
    println!("Guess the number!");
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        let result: Result<usize, Error> = io::stdin().read_line(&mut guess);
        result.expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Match any error
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}