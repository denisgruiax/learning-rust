extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number: ");

    let secret_number = rand::thread_rng().gen_range(1, 6);

    let mut text = String::new();

    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");

    let result: Option<i32> = text.trim().parse().ok();

    let your_guess = match result {
        Some(num) => num,
        None => 0,
    };

    println!("The secret number is: {}", secret_number);

    match secret_number == your_guess {
        true => println!("You guessed the number"),
        false => println!("Wrong guess...")
    };
}
