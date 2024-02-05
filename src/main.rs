fn main() {
    let mut pontuacao: i32 = 0;
    let escolha: i32 = 1;

    println!("Bem-vindo à floresta misteriosa!");
    println!("Escolha sua próxima ação");
    println!("1 - Entrar na caverna escura");
    println!("2 - Seguir pelo caminho iluminado");
    println!("3 - Cruzar a ponte frágil");
    println!("4 - Descansar na beira do riacho");

    if escolha == 1 {
        pontuacao += 50;
    }
    if escolha == 2 {
        pontuacao -= 20;
    }
    if escolha == 3 {
        pontuacao -= 20;
    }
    if escolha == 4 {
        pontuacao += 10;
    }

    println!("Sua escolha foi {}", escolha);
    println!("Sua pontuação foi {}", pontuacao);
}
