use rand::prelude::*;
use std::io;

fn main() {
    loop {
        play();

        println!("Would you like to play again? (Y/N)");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Huh!?");

        if again.trim().to_uppercase() == "N" {
            break;
        }
    }
}

fn read_guess() -> Result<i32,std::num::ParseIntError> {
    let mut guess_str = String::new();

    io::stdin().read_line(&mut guess_str).expect("What!?");

    let mut guess_option = guess_str.trim().parse();

    while let Err(_) = guess_option {
        println!("You must enter a valid number between 1 and 100.");

        guess_str.clear();
        io::stdin().read_line(&mut guess_str).expect("What!?");
        guess_option = guess_str.trim().parse();
    }

    guess_option
}

fn play() {
    let number = rand::thread_rng().gen_range(1, 101);
    let mut num_guesses = 0;

    println!("I have a number between 1 and 100.");

    loop {
        let guess = read_guess().unwrap();

        if guess < 1 || guess > 100 {
            println!("Number should be between 1 and 100")
        } else {
            num_guesses = num_guesses + 1;

            if guess > number {
                println!("Nope! My number is lower!");
            } else if guess < number {
                println!("Wrong! My number is higher!");
            } else if guess == number {
                println!("Yes! You guessed it in {} tries!", num_guesses);
                break;
            }
        }

    }


}
