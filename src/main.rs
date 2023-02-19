use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // Set the range for the game.
    
    let low = 1;
    let high = 10;

    println!("Welcome to Geoff's Guessing Game!");
    println!("Please choose a range, lowest first")

    let secret_number = rand::thread_rng().gen_range(low..=high);
    let mut guess = String::new();

    loop {
        println!("Guess a number between {} and {} (inclusive).", low, high);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} is too small!", guess),
            Ordering::Greater => println!("{} is too big!", guess),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}