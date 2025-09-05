use std::io;

fn adicione_na_media(x: f32) -> f32 {
    static mut SOMA: f32 = 0.0;
    unsafe {
        SOMA += x;
        return SOMA;
    }
}

fn main() {
    let mut contador = 0;
    println!("Digite quantas vezes vc quer rodar esse código:");
    let mut qtd = String::new();
    io::stdin().read_line(&mut qtd).expect("Falha ao ler!");
    let qtd: u32 = qtd.trim().parse().expect("Deu ruim.");

    for _i in 0..qtd {
        println!("Digite um valor:");
        let mut numero = String::new();
        io::stdin().read_line(&mut numero).expect("Falha ao ler!");
        let numero: f32 = numero.trim().parse().expect("Deu ruim.");
        let atual = adicione_na_media(numero);
        contador += 1;
        println!("Numero atual: {atual}");
        println!("Média dos valores: {}", atual / contador as f32);
    }
}
