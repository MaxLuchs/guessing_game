use rand::Rng;
use std::io::stdin;

fn main() {
    let secret = rand::thread_rng().gen_range(1..100);
    println!("Guess a secret number...");
    fn guess_secret(secret: i32) {
        let stdin = stdin();
        let mut buffer = String::new();
        stdin.read_line(&mut buffer);
        buffer.trim().parse::<i32>().map(|number| {
            if number == secret {
                println!("Match ... Game over");
            } else if number < secret {
                println!("Too low... try again");
                guess_secret(secret);
            } else if number > secret {
                println!("Too high... try again");
                guess_secret(secret);
            }
        });
    }
    guess_secret(secret);
}
