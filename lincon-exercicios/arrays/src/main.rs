use std::io;

fn main() {
    // Questão 1
    let lista = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let mut soma = 0;

    for i in 0..10 {
        println!("Posição {}: {}", i, lista[i]);
    }
    println!("");
    for i in (0..10).rev() {
        println!("Posição {}: {}", i, lista[i]);
    }
    println!("\nQuinta posição: {}\n", lista[4]);

    for i in 0..10 {
        if i % 2 != 0 {
            println!("Posição {}: {}", i, lista[i]);
        }
    }

    for i in 0..10 {
        soma += lista[i];
    }
    println!("\nSoma: {}", soma);

    // Questão 2

    let mut lista = [0;10];
    let mut soma = 0;

    for i in 0..10 {
        let mut numero_atual = String::new();
        io::stdin().read_line(&mut numero_atual).expect("Falha de tipo");
        let numero_atual: i32 = numero_atual.trim().parse().expect("Erro");
        lista[i] = numero_atual;
    }

    for i in 0..10 {
        println!("Posição {}: {}", i, lista[i]);
    }
    println!("");
    for i in (0..10).rev() {
        println!("Posição {}: {}", i, lista[i]);
    }
    println!("\nQuinta posição: {}\n", lista[4]);

    for i in 0..10 {
        if i % 2 != 0 {
            println!("Posição {}: {}", i, lista[i]);
        }
    }

    for i in 0..10 {
        soma += lista[i];
    }
    println!("\nSoma: {}", soma);

    // Questão 3

    let mut lista = [0;10];
    let (mut soma, mut contador) = (0,0);

    for i in 0..10 {
        let mut numero_atual = String::new();
        io::stdin().read_line(&mut numero_atual).expect("Falha de tipo");
        let numero_atual: i32 = numero_atual.trim().parse().expect("Erro");
        lista[i] = numero_atual;
    }

    for i in 0..10 {
        println!("Posição {}: {}", i, lista[i]);
    }
    println!("");
    for i in (0..10).rev() {
        println!("Posição {}: {}", i, lista[i]);
    }
    println!("\nQuinta posição: {}\n", lista[4]);

    for i in 0..10 {
        if i % 2 != 0 {
            println!("Posição {}: {}", i, lista[i]);
        }
    }

    for i in 0..10 {
        soma += lista[i];
        contador += 1;
    }
    println!("\nSoma: {}", soma);
    println!("\nMédia: {}", soma/contador);

    // Questão 4

    let mut lista = [0;10];
    let (mut soma, mut contador) = (0,0);

    for i in 0..10 {
        let mut numero_atual = String::new();
        io::stdin().read_line(&mut numero_atual).expect("Falha de tipo");
        let numero_atual: i32 = numero_atual.trim().parse().expect("Erro");
        lista[i] = numero_atual;
    }

    for i in 0..10 {
        println!("Posição {}: {}", i, lista[i]);
    }
    println!("");
    for i in (0..10).rev() {
        println!("Posição {}: {}", i, lista[i]);
    }
    println!("\nQuinta posição: {}\n", lista[4]);

    for i in 0..10 {
        if i % 2 != 0 {
            println!("Posição {}: {}", i, lista[i]);
        }
    }

    for i in 0..10 {
        soma += lista[i];
        contador += 1;
    }
    println!("\nSoma: {}", soma);
    println!("\nMédia: {}", soma/contador);

    let mut valor = String::new();
    io::stdin().read_line(&mut valor).expect("Digite um número.");
    let valor: i32 = valor.trim().parse().expect("Falha ao converter");
    
    for i in 0..10 {
        if valor == lista[i] {
            println!("Valor encontrado na posição {}", i);
            break;
        } else {
            println!("Valor não encontrado.")
        }
    }

    // Questão 5

    let mut lista = [0;10];
    let (mut soma, mut contador, mut maior) = (0,0,0);

    for i in 0..10 {
        let mut numero_atual = String::new();
        io::stdin().read_line(&mut numero_atual).expect("Falha de tipo");
        let numero_atual: i32 = numero_atual.trim().parse().expect("Erro");
        lista[i] = numero_atual;
    }

    for i in 0..10 {
        println!("Posição {}: {}", i, lista[i]);
    }
    println!("");
    for i in (0..10).rev() {
        println!("Posição {}: {}", i, lista[i]);
    }
    println!("\nQuinta posição: {}\n", lista[4]);

    for i in 0..10 {
        if i % 2 != 0 {
            println!("Posição {}: {}", i, lista[i]);
        }
    }

    for i in 0..10 {
        soma += lista[i];
        contador += 1;
    }
    println!("\nSoma: {}", soma);
    println!("\nMédia: {}", soma/contador);

    let mut valor = String::new();
    io::stdin().read_line(&mut valor).expect("Digite um número.");
    let valor: i32 = valor.trim().parse().expect("Falha ao converter");
    
    for i in 0..10 {
        if valor == lista[i] {
            println!("Valor encontrado na posição {}", i);
            break;
        } else {
            println!("Valor não encontrado.")
        }
    }

    for i in 0..10 {
        if i == 0 {
            maior = lista[i];
        }
        if lista[i] > maior {
            maior = lista[i];
        }
    }
    println!("Maior valor: {}", maior);
}
