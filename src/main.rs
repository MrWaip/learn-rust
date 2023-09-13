use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    loop {


        println!("Please input your guess. Input \"quite\" for exit.");

        guess.clear();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "quite" {
            return;
        }


        let guess: u32 = match guess.trim().parse() {
            Err(_) => {
                println!("LoL it not number");
                continue;
            }
            Ok(v) => v,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("Kek u got it {guess}");
                break;
            }
        }
    }
}
