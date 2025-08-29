use std::io;

fn adicione_na_media(x: f32) -> f32{
    static mut SOMA: f32 = 0.0;
    unsafe {
        SOMA += x;
        return SOMA;
    }
}

fn main() {
    let mut contador = 0.0;
    loop {
    println!("Digite um valor:");
    let mut numero = String::new();
    io::stdin().read_line(&mut numero).expect("Falha ao ler!");
    let numero: f32 = numero.trim().parse().expect("Deu ruim.");
    let atual = adicione_na_media(numero);
    if numero < 0.0 {
        break;
    }
    contador += 1.0;
    println!("Numero atual: {atual}");
    println!("Média dos valores: {}", atual/contador);
    }
}
