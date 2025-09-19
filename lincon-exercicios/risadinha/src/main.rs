use std::io;

fn main() {
    let mut risada = String::new();
    let mut vogais = String::new();
    let mut vogais_invertidas = String::new();

    io::stdin().read_line(&mut risada).expect("Erro ao ler");

    let caracteres: Vec<char> = risada.trim().chars().collect();
    let mut vogais: Vec<char> = vogais.trim().chars().collect();
    let mut vogais_invertidas: Vec<char> = vogais_invertidas.trim().chars().collect();

    let qtd1: usize = caracteres.len();

    for i in 0..qtd1 {
        if caracteres[i] == 'a'
            || caracteres[i] == 'e'
            || caracteres[i] == 'i'
            || caracteres[i] == 'o'
            || caracteres[i] == 'u'
        {
            vogais.push(caracteres[i]);
        }
    }

    let qtd2: usize = vogais.len();

    for i in (0..qtd2).rev() {
        vogais_invertidas.push(vogais[i]);
    }

    for i in 0..qtd2 {
        if vogais[i] == vogais_invertidas[i] {
        } else {
            println!("N");
            return ();
        }
    }

    println!("S");
}
