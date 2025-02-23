use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    // Generate random number using rand library and importing Rng is necessary
    let screct_number = rand::thread_rng().gen_range(1..=100);
    println!("Generated Secret Num {screct_number}");

    // loop is the keyword to run program to run until it break;
    loop {
        println!("Please input the number:");

        // input from user
        let mut guess = String::new();
        // Read user data amd muttable so it can override variable
        io::stdin().read_line(&mut guess).expect("failed");

        // converting to integare;
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Please enter valid number");
                continue;
            }
        };

        // match keyword compass and need to handle all the possible outcome
        match guess.cmp(&screct_number) {
            Ordering::Equal => {
                println!("You Won");
                break;
            },
            Ordering::Greater => println!("Too Big"),
            Ordering::Less => println!("Too Small"),
        }
    }
}
