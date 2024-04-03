use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    loop {
        let secret_number = rand::thread_rng().gen_range(0..101);

        println!("Welcome to Guessing Game!");

        loop {
            println!("Enter your guess number");

            let mut guess_number = String::new();

            // &mut guess_number is a mutable reference
            // `&` represents that we are passing reference of the guess_number to the function.
            // `mut` represents that reference can be mutated
            io::stdin()
                .read_line(&mut guess_number)
                .expect("Unable to read line!");

            let guess_number: u32 = match guess_number.trim().parse() {
                Ok(number) => number,
                Err(_) => {
                    println!("Enter valid number");
                    continue;
                }
            };

            match guess_number.cmp(&secret_number) {
                Ordering::Less => println!("Your guess too small!"),
                Ordering::Greater => println!("Your guess too big!"),
                Ordering::Equal => {
                    println!("Guess what, You win!");
                    break;
                }
            }
        }

        println!("1. Play again");
        println!("2. Exit game");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Unable to read line!");

        let option: u32 = match option.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Enter valid number");
                continue;
            }
        };

        if option == 2 {
            break;
        }
    }
}
