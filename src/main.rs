use std::io::stdin;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut count: i16 = 1;

    while count<=10{
        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("Enter your guess: ");
        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .expect("Failed");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("Bingo");
                break;
            }
            Ordering::Greater => println!("Too Big!"),
        }
        println!("Tries left: {}", 10-count);
        count += 1;
        if count == 11 {
            println!("Outta tries!");
        }
    }

}
