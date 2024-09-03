use std::io;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut score = 0;

    println!("Welcome to the logarithm game!");
    println!("Enter a number to solve the equation. To exit, enter 'exit'.");

    loop {
        let base: u32 = rng.gen_range(2..11); 
        let value: u32 = rng.gen_range(1..101); 

        let correct_answer = (value as f64).log(base as f64);
        let formatted_base = base as f64;
        let formatted_value = value as f64;

        println!("Find the logarithm of {:.2} with base {:.2}: ", formatted_value, formatted_base);

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Couldn't read input.");

        let trimmed_input = user_input.trim();
        if trimmed_input.to_lowercase() == "exit" {
            break;
        }

        let user_answer: f64 = match trimmed_input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }

        };

        if (user_answer - correct_answer).abs() < 0.01 {
            score += 1;
            println!("True. Your score: {}", score);
        } else {
            println!("False. Correct answer: {:.2}", correct_answer);
        }
    }

    println!("Your final score: {}", score);
}
