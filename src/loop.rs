use rand::Rng;
use std::io::stdin;

fn main() {
    let secret = rand::thread_rng().gen_range(1..100);
    println!("[Use loop] Guess a secret number...");
    let mut matched = false;
    while !matched {
        let stdin = stdin();
        let mut buffer = String::new();
        stdin.read_line(&mut buffer);
        let result = buffer.trim().parse::<i32>().map(|number| {
            if number == secret {
                println!("Match ... Game over");
                matched = true;
            } else if number < secret {
                println!("Too low... try again");
            } else if number > secret {
                println!("Too high... try again");
            }
        });
        if let Err(error) = result {
            println!("Not a number... try again");
        }
    }
}


