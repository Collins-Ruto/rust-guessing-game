use std::io;

fn main() {
    println!("Hello, world!");

    println!("Hello and welcome to my guessing game!");
    println!("Please enter your guess below: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Something went wrong");

    println!("you guesses : {}", guess);
}
