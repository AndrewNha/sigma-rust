use std::io::{self};

fn validator(chars: &[char]) -> bool{
    let mut maisculas = 0;
    let mut minusculas = 0;
    let mut numeros = 0;
    let qtd = chars.len();

    if qtd >= 6 && qtd <= 32 {
    } else {
        return false;
    }

    for i in 0..qtd{
        if !chars[i].is_ascii() {
            return false;
        }
        if chars[i].is_numeric() == false && chars[i].is_lowercase() == false &&  chars[i].is_uppercase() == false{
            return false;
        }
        if chars[i].is_uppercase() {
            maisculas += 1;
        }
        if chars[i].is_lowercase() {
            minusculas += 1;
        }
        if chars[i].is_numeric(){
            numeros += 1;
        }
    }
    if maisculas >= 1 && minusculas >= 1 && numeros >= 1 {
    } else{
        return false;
    }

    return true;
}
fn main() {
    loop{
        let mut senha = String::new();
        
        io::stdin()
            .read_line(&mut senha)
            .expect("Erro na leitura."); 

        if senha.is_empty() {
            break;
        }

        let chars: Vec<char> = senha.trim().chars().collect();

        if validator(&chars){
            println!("Senha valida.")
        } else {
            println!("Senha invalida.")
        }
    }
}
