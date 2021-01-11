use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: i32 = rand::thread_rng().gen();

    println!("The Secret is {}",secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}