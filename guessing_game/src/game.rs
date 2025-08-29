use std::io;
use std::cmp::Ordering; // biblioteca para comparar valores
use rand::Rng; // biblioteca para gerar numeros aleatorios

fn main() {
    println!("Guess the number!");

    println!("Please input your guess, NOW:");

    let secret_number = rand::thread_rng().gen_range(1..=100); // de 1 até 100, incluindo o 100

    println!("Secret number is: {secret_number}");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // se der erro, printa a msg

    let guess: u32 = guess
    .trim() // remove espaços em branco
    .parse() // converte string para numero (unsigned 32 bits)
    .expect("Please type a number!"); // se der erro, printa a msg

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

}