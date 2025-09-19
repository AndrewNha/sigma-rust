fn main() {
    let mut tupla_massa = ("banana", 4, 5.0);
    // tupla_massa.0 = 4; não daria certo, tem que ser o mesmo tipo de antes
    println!("A tupla é: {}", tupla_massa.0);
    println!("A tupla é: {}", tupla_massa.1);
    println!("A tupla é: {}", tupla_massa.2);
    let (a,b,c) = tupla_massa; // definir variavel pra cada posição
    println!("O valor de b é: {}", b);
    let listinha = [0;5];
    println!("Listinha 1 : {}", listinha[3]);
}
