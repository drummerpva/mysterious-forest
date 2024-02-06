use std::io::stdin;

fn main() {
    let mut pontuacao: i32 = 0;
    let escolha: u32;

    println!("Bem-vindo à floresta misteriosa!");
    println!("Escolha sua próxima ação");
    println!("1 - Entrar na caverna escura");
    println!("2 - Seguir pelo caminho iluminado");
    println!("3 - Cruzar a ponte frágil");
    println!("4 - Descansar na beira do riacho");

    let mut escolha_str = String::new();
    stdin().read_line(&mut escolha_str).unwrap();
    escolha = escolha_str
        .trim()
        .parse()
        .expect("Dado informado inválido!");

    if escolha == 1 {
        pontuacao += 50;
    } else if escolha == 2 {
        pontuacao -= 20;
    } else if escolha == 3 {
        pontuacao -= 20;
    } else if escolha == 4 {
        pontuacao += 10;
    }

    println!("Sua escolha foi {}", escolha);
    println!("Sua pontuação foi {}", pontuacao);
}
