use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");
    println!("Please input your guess");

    // secret's inferred type of u32 is because of the "guess.cmp(&secret)" declaration
    let secret = rand::thread_rng().gen_range(1..100);

    // println!("The secret number is {secret}");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");
        // match takes the expression "guess.cmp(&secret)" and sets
        // up the control flow based on pattern matching
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"), // each pattern matcher is called an "arm"
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        };
    }
}
