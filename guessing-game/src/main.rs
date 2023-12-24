
use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 10);
    println!("Please enter a number between 0 - 9");

    loop {
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess : u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter a valid number".red());
                continue;
            },
        };

        if guess > 9{
            println!("{}", "Incorrect input range, please enter again".red());
            continue;
        }

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}", "Too Small".red()),
            Ordering::Greater => println!("{}", "Too Big".red()),
            Ordering::Equal => {
                println!("{}", format!("You Won, the secret number was: {}", secret_number).green());
                break;
            }
        }
    }
}
