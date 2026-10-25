use colored::*;
use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Gussing Game");
    let secret = rand::rng().random_range(1..101);

    loop {
        println!("Enter a number: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error getting user number.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Equal => {
                println!("{}", "Equals, Won".green());
                break;
            }
            Ordering::Less => println!("{}", "Less".red()),
            Ordering::Greater => println!("{}", "Greater".red()),
        }
    }
}
