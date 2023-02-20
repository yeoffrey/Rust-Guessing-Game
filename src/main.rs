use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Set the range for the game.
    let mut low = String::new();
    let mut high = String::new();

    println!("Welcome to Geoff's Guessing Game!");

    // Set upper range.
    println!("Choose your highest number (inclusive).");
    io::stdin().read_line(&mut high).expect("Failed to read line");
    let high: u32 = match high.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };

    // Set lower range.
    println!("Choose your lowest number (inclusive).");
    io::stdin().read_line(&mut low).expect("Failed to read line");
    let low: u32 = match low.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };

    // Initialize the global game variables.
    let secret_number = rand::thread_rng().gen_range(low..=high);
    let mut tries: u32 = 0;

    loop {
        println!("Guess a number between {} and {} (inclusive).", low, high);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        tries += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} is too small!", guess),
            Ordering::Greater => println!("{} is too big!", guess),
            Ordering::Equal => {
                println!("You win! The number was {}. It took you {} tries.", secret_number, tries);
                break;
            }
        }
    }
}