use std::cmp::Ordering;
use std::io;
use std::process::exit;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess. Press enter to exit the game");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get user input");

        if guess == "\n" {
            println!("Exiting the game");
            exit(1);
        }
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! Closing the game");
                break;
            }
        }
    }
}
