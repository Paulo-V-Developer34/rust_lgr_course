use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let naleatorio = rand::thread_rng().gen_range(1, 101); //gera números aleatórios

    println!("Advinhe o número");

    loop {
        println!("Por favor insira um número");

        let mut tentativa = String::new(); //sinceramente não sei o porquê da utilização do "String::new()" //No vídeo o professor disse que essa é uma "growable string"(não sei o que isso significa) //também há mensão de que "::" serve para associação e não para acesso a methodos estáticos como em PHP

        io::stdin()
            .read_line(&mut tentativa) //aqui eu não pude utilizar ";" pois o código seria extendido com o "." //aqui é utilizado o conceito de "buff" (não sei o que é)
            .expect("failed to read line");

        let tentativa: u32 = tentativa.trim().parse(); //trim removerá espaços em branco //parse fará algo que eu não sei o que é// u32 é um número "não assinado"

        println!("Sua tentativa foi {}", tentativa);

        match tentativa.cmp(&naleatorio) {
            Ordering::Less => println!("Número muito pequeno"),
            Ordering::Greater => println!("Número muito grande"),
            Ordering::Equal => {
                println!("Você acertou! O número aleatório era {}!", naleatorio);
                break;
            }
        }
    }
}
