use std::io;
fn main() {
    let mut casos = String::new();
    io::stdin().read_line(&mut casos).expect("Erro ao ler");
    let casos: i32 = casos.trim().parse().expect("Digite um numero válido");

    leds(casos);
}

fn leds(casos: i32) {
    for i in 0..casos{
        let mut contador = 0;
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Erro ao ler");
        let caracteres: Vec<char> = num.trim().chars().collect();
        let tamanho = caracteres.len();
        for j in 0..tamanho{
            match caracteres[j] {
                '1' => contador+=2,
                '2' => contador+=5,
                '3' => contador+=5,
                '4' => contador+=4,
                '5' => contador+=5,
                '6' => contador+=6,
                '7' => contador+=3,
                '8' => contador+=7,
                '9' => contador+=6,
                '0' => contador+=6,
                _ => {}
            }
        }
        println!("{} leds", contador);
    }
}