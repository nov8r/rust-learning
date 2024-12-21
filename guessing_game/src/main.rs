use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            // &mut means "I'm lending you permission to modify guess."
            // read_line needs to be able to modify guess so it needs the mutable refference
            .read_line(&mut guess)
            .expect("Failed to read line");

        // If parse() successfully converts the input into a number, 
        // it assigns that number to guess.
        // If parse() fails (e.g., the user typed something non-numeric), 
        // it matches Err(_) and calls continue to restart the loop.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

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