use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the Guess the Number Game!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("I'm thinking of a number between 1 and 100. Can you guess it?");
    println!("You have one chance to guess the number!");

    // Read the user's guess
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    // Convert the guess to a number
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    // Check the guess
    if guess == secret_number {
        println!("Congratulations! You guessed the correct number: {}", secret_number);
    } else if guess < secret_number {
        println!("Too low! The correct number was: {}", secret_number);
    } else {
        println!("Too high! The correct number was: {}", secret_number);
    }

    println!("Game Over. Thanks for playing!");
}
