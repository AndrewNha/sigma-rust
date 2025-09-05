fn main() {
    let soma = 4 + 5;
    println!("A soma de 4 + 5 é: {}", soma);
    let brutal = 67.65 * 2.3;
    println!("O resultado de 67.65 * 2.3 é: {:07.5}", brutal); // :(numero minimo, de algarismos contando com o ponto).(decimais)
    let resto = 43 % 5;
    println!("O resto de 43 % 5 é: {}", resto);
    let bagulho: bool = true;
    println!("O valor booleano é: {}", bagulho);
    let sigma_lobo: char = '🐺'; // aspas simples pq é caractere
    println!("O caractere é: {}", sigma_lobo);
    let mut tupla_massa = ("banana", 4, 5.0);
    // tupla_massa.0 = 4; daria
    println!("A tupla é: {}", tupla_massa.0);
    println!("A tupla é: {}", tupla_massa.1);
    println!("A tupla é: {}", tupla_massa.2);
}
