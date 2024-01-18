use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..1001);
    println!("The secret number is: {}", secret_number);

    loop{
        println!("Enter Number between 1 and 1000: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Number only please!".red());
                continue;
            },
        };

        println!("You entered: {}", input);

        match input.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small!".red()),
            Ordering::Greater => println!("{}", "Too Big!".red()),
            Ordering::Equal => {
                println!("{}", "You Win!".green());
                break;
            },
        }
    }   
}
