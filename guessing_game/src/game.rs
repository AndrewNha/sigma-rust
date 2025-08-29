use std::io;
use std::cmp::Ordering; // serio que tem q usar biblioteca pra porra do if.
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess, NOW:");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number is: {secret_number}");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

}