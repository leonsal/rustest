use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is:{}", secret_number);

    loop {
        // Reads guess from standard input
        println!("Input your guess:");
        let mut guess = String::new();
        let nread = io::stdin()
            .read_line(&mut guess)
            .expect("error reading line");

        // Converts read string to number
        // Shadowing previously 'guess' variable
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed (nread={}): {guess}", nread);

        // Checks the guessed number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("RIGHT!");
                break;
            }
        }
    }
}
