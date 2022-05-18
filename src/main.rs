use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello and welcome to my guessing game!");

    let random_num = rand::thread_rng().gen_range(1..101);

    loop{
        println!("Please enter your guess below: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Something went wrong");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed : {}", guess);

        match guess.cmp(&random_num) {
            Ordering::Less => println!("Guess higher"),
            Ordering::Greater => println!("Guess lower"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
