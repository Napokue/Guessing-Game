extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng()
        .gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        
        let (is_number, guess) = is_guess_a_number(&mut let_user_guess());
        if !is_number {
            continue;
        }

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn let_user_guess() -> String {
    let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line!");
    return guess;
}

fn is_guess_a_number(guess: &mut String) -> (bool, u32) {
    match guess.trim().parse() {
        Ok(num) => {
            return (true, num);
        },
        Err(_) => {
            println!("Input is not a number, please try again!");
            return (false, 0);
        }
    }; 
}

