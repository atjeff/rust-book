use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, Welcome to the guessing game!");

    let random = rand::thread_rng().gen_range(1..=100);

    // This is a loop you have to purposefully break out of
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Because guess is a string of input above, we want to parse as u32 so that we can compare it
        // Redeclaring this variable is called shadowing, which I don't think I'm a fan of.
        // "This pattern is often used when you want to convert a value from one type to another type."
        let guess: u32 = match guess.trim().parse() {
            // Here we're doing a switch / match statement on the Result type. Loop again if they do something dumb
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&random) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                println!("You win! GG");

                break;
            }
        }
    }
}
