use std::io;

// Escreva um programa que declare uma variável constante chamada π (pi) com o
// valor 3,14159. Em seguida, peça ao usuário para digitar o raio de um círculo e calcule
// e exiba a  área desse círculo
fn exercicio1() {
    // Cria constante PI com o valor solicitado
    const PI: f64 = 3.14159;

    // Cria uma variável do tipo String para armazenar a entrada do usuário
    let mut buffer: String = "".to_string();

    println!("Digite o raio da circunferência");
    // Joga os dados da entrada padrão (stdin) para o buffer criado
    io::stdin().read_line(&mut buffer).unwrap();

    // Converte as informações em String para um f64
    let raio_float: f64 = buffer.trim().parse().unwrap();

    println!("A área do círculo de raio {} é {}", raio_float, PI * (raio_float * raio_float));
}

/* Escreva um programa que declare uma variável mutável chamada contador e a inicialize
com o valor 1. Em seguida, use um while para incrementar o valor do contador até que
ele atinja 10. Exiba o valor do contador a cada iteração. */
fn exercicio2() {
    let mut contador: i32 = 1;

    // Enquanto o valor de contador for menor ou igual a 10
    while contador <= 10 {
        // Mostre na tela "Contador=<VALOR DE CONTADOR>"
        println!("Contador={}", contador);

        // Incrementa 1 na variável contador
        contador += 1;
    }
}

/* Escreva um programa que declare uma variável chamada soma e a inicialize
com o valor 0. Em seguida, use um laço for para somar os números de 1 a 10 à
variável soma. Ao final, exiba o valor da soma. */
fn exercicio3() {
    let mut soma: i32 = 0;

    // Criamos uma variável 'numero' para representar o contador 
    // do 'range' indo de 1 até 10 (1..10 iria de 1 até 9)
    for numero in 1..=10 {
        soma += numero;
    }

    println!("A soma dos números de 1 a 10 é {}", soma);
}

/* Escreva um programa que solicite ao usuário para digitar um número inteiro.
Em seguida, use um laço de repetição for para exibir todos os números de 1 até o
número digitado pelo usuário. */
fn exercicio4() {
    // Cria uma variável do tipo String para armazenar a entrada do usuário
    let mut buffer: String = "".to_string();

    println!("Digite um número");
    // Joga os dados da entrada padrão (stdin) para o buffer criado
    io::stdin().read_line(&mut buffer).unwrap();

    // Converte as informações em String para um i32
    let numero: i32 = buffer.trim().parse().unwrap();

    // Criamos uma variável 'n' para representar o contador 
    // do 'range' indo de 1 até numero (1..numero iria de 1 até numero-1)
    for n in 1..=numero {
        println!("n={}", n);
    }
}

/* Escreva um programa que peça ao usuário para digitar um número inteiro positivo.
Em seguida, use um laço de repetição for para exibir todos os números pares de 2 até o
número digitado pelo usuário. */
fn exercicio5() {
    // Cria uma variável do tipo String para armazenar a entrada do usuário
    let mut buffer: String = "".to_string();

    println!("Digite um número");
    // Joga os dados da entrada padrão (stdin) para o buffer criado
    io::stdin().read_line(&mut buffer).unwrap();

    // Converte as informações em String para um i32
    let numero: i32 = buffer.trim().parse().unwrap();

    // Criamos uma variável 'n' para representar o contador 
    // do 'range' indo de 1 até numero (1..numero iria de 1 até numero-1)
    for n in 1..=numero {
        // Verifica se o número é divisível por 2 e então mostrar o número na tela
        if n % 2 == 0 {
            println!("n={}", n);
        }
    }
}

/* Escreva um programa que peça ao usuário para digitar uma senha. Se a senha digitada
for "12345" exiba a mensagem "Acesso concedido". Caso contrário, exiba a mensagem "Senha incorreta". */
fn exercicio6() {
    // Cria uma variável do tipo String para armazenar a entrada do usuário
    let mut buffer: String = "".to_string();

    println!("Digite sua senha");
    // Joga os dados da entrada padrão (stdin) para o buffer criado
    io::stdin().read_line(&mut buffer).unwrap();

    // Removendo a quebra de linha (trim) que retorna junto à leitura da linha
    buffer = buffer.trim().to_string();

    // Verificando se o valor armazenado no buffer é igual a uma string "12345".
    // Também podemos implementar buffer == "12345"
    if buffer.eq("12345") {
        println!("Acesso concedido!");
    } else {
        println!("Senha incorreta.");
    }
}

/* Escreva um programa que declare uma variável chamada IDADE_MINIMA com o valor 18.
Em seguida, peça ao usuário para digitar sua idade. Se a idade digitada for maior
ou igual a IDADE_MINIMA, exiba a mensagem "Você é maior de idade", caso contrário, 
exiba a mensagem "Você é MENOR de idade!". */
fn exercicio7() {
    // Cria uma variável do tipo String para armazenar a entrada do usuário
    let mut buffer: String = "".to_string();
    const IDADE_MINIMA: i32 = 18;

    println!("Digite sua idade");
    // Joga os dados da entrada padrão (stdin) para o buffer criado
    io::stdin().read_line(&mut buffer).unwrap();

    // Remove quebra de linha da entrada
    buffer = buffer.trim().to_string();

    // Converte texto em um i32
    let idade: i32 = buffer.parse().unwrap();

    // Verifica se idade é menor que IDADE_MÍNIMA
    if idade < IDADE_MINIMA {
        println!("Você é MENOR de idade!");
    } else {
        println!("Você é maior de idade.");
    }
}

/* Escreva um programa que peça ao usuário para digitar um número inteiro positivo.
Em seguida, use um laço de repetição while para calcular e exibir o fatorial desse número */
fn exercicio8() {
    // Cria uma variável do tipo String para armazenar a entrada do usuário
    let mut buffer: String = "".to_string();

    println!("Digite um número");
    // Joga os dados da entrada padrão (stdin) para o buffer criado
    io::stdin().read_line(&mut buffer).unwrap();

    // Remove quebra de linha da entrada
    buffer = buffer.trim().to_string();

    // Converte texto em um i32
    let n: i32 = buffer.parse().unwrap();

    let mut fatorial: i32 = 1;
    let mut contador: i32 = 1;

    // Enquanto o contador for menor ou igual a n faça:
    while contador <= n {
        fatorial = fatorial * contador;
        contador += 1;
    }
    println!("O fatorial de {} é {}", n, fatorial);
}

/* Escreva um programa que declare uma variável chamada numero e a inicialize com o valor 10.
Use um laço de repetição while para subtrair 2 de número até que ele seja menor ou igual a 0.
Exiba o valor de  'numero' a cada iteração. */
fn exercicio9() {
    let mut numero:i32 = 10;
    while numero > 0 {
        println!("numero={}", numero);
        numero -= 2;
    }
}

/* Escreva um programa que peça ao usuário para digitar um número inteiro. Em seguida,
use um laço de repetição for para verificar se o número é primo ou não. Exiba uma mensagem
indicando se o número é primo ou não */
fn exercicio10() {
    // Cria uma variável do tipo String para armazenar a entrada do usuário
    let mut buffer: String = "".to_string();

    println!("Digite um número");
    // Joga os dados da entrada padrão (stdin) para o buffer criado
    io::stdin().read_line(&mut buffer).unwrap();

    // Remove quebra de linha da entrada
    buffer = buffer.trim().to_string();

    // Converte texto em um i32
    let n: i32 = buffer.parse().unwrap();

    for num in 2..=n {
        if primo(num) {
            println!("Número {} é primo", num);
        } else {
            println!("Número {} NÃO é primo", num);
        }
    }
}

fn primo(num: i32) -> bool {
    for i in 2..(num/2+1) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    // exercicio1();
    // exercicio2();
    // exercicio3();
    // exercicio4();
    // exercicio5();
    // exercicio6();
    // exercicio7();
    // exercicio8();
    // exercicio9();
    // exercicio10();
}
