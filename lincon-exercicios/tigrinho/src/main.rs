use rand::Rng;
use std::io;

fn main() {
    let emojis = ['🍒', '🍋', '🍊', '🍉', '🍇', '💎', '🍓'];
    println!("Bem-vindo ao Fortune Tiger 🐯: Rust Edition\nAperte 'ENTER' para continuar");
    loop {
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Erro ao ler");
        let entrada = entrada.trim();

        if entrada == "Q" || entrada == "q" {
            return;
        } else {
            if entrada.is_empty() {
            } else {
                println!("Erro: digite certo.");
                return;
            }
        }

        let slot1 = rand::rng().random_range(1..=6);
        let slot2 = rand::rng().random_range(1..=6);
        let slot3 = rand::rng().random_range(1..=6);

        println!("---------------------------");
        println!(
            "-----| {} | {} | {} |-----",
            emojis[slot1], emojis[slot2], emojis[slot3]
        );
        println!("---------------------------");

        if emojis[slot1] == emojis[5] && emojis[slot1] == emojis[slot2] && emojis[slot1] == emojis[slot3] {
            println!("🎆🎆🎆🎆🎆 !!!!!BIG WIN!!!!! 🎆🎆🎆🎆🎆");
        } else {
            println!("Talvez na próxima.")
        }

        println!("Pressione 'ENTER' para continuar. Pressione 'Q' e 'ENTER' para terminar.");
    }
}
