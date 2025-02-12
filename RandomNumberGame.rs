use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to the Number Guessing Game!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;
    
    loop {
        println!("Enter your guess (1-100):");
        
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        
        attempts += 1;
        
        if guess < secret_number {
            println!("Too low! Try again.");
        } else if guess > secret_number {
            println!("Too high! Try again.");
        } else {
            println!("Congratulations! You guessed the number in {} attempts.", attempts);
            break;
        }
    }
}
