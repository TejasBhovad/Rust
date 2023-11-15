use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the Number!");

    let secret_no = rand::thread_rng().gen_range(1..=10);
    println!("secret no: {}", secret_no);

    loop {
        println!("Enter number to guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadowing: change type of var w/o changing name
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed: {}", guess);

        match guess.cmp(&secret_no) {
            Ordering::Less => println!("{}", "Too Small".red()),
            Ordering::Greater => println!("{}", "Too Big".red()),
            Ordering::Equal => {
                println!("{}", "You Guess Correctly".green());
                break;
            }
        }
    }
}
