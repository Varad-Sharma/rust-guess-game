use std::io::stdin;
use std::cmp::Ordering;
use rand::Rng;
use colored::Colorize;

fn main() {
    let mut count: i16 = 1;
    let secret_number = rand::thread_rng().gen_range(1..=100);

    while count<=10{
        println!("--------------------------------------------------------------------------------------");

        println!("{}", "Enter your guess: ".green().italic());
        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .expect("Failed");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("{}", "You guessed: ".bright_yellow());
        println!("{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small!".bright_red().italic()),
            Ordering::Equal => {
                println!("{}", "Bingo".green().bold());
                break;
            }
            Ordering::Greater => println!("{}", "Too Big!".red().bold()),
        }
        print!("{}", "Tries Left: ".yellow());
        println!("{}", 10-count);
        count += 1;
        if count == 11 {
            println!("{}", "Outta tries!".bright_red().bold());
        }
    }

}
