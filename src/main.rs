// Programa para treinar CLI's in Rust
// task add <description>
// task list <all/todo/done/in-progress>
// task delete <id>
// task update <id> <description>
// task mark-done <id>
// task mark-in-progress <id>

use std::env;

fn adicionar(desc: &str) -> String {
    format!("Tarefa \"{desc}\" adicionada com sucesso! ID: <id>")
}

fn main() {
    //println!("====================== GERENCIE SUAS TAREFAS =============================\n\n\n\n");
    //Pegar os argumentos passados para o programa 'argv[i]'
    let args: Vec<String> = env::args().collect(); //

    //Verifica se tem a quantidade de argumentos necessária para rodar o programa
    if args.len() < 3 {
        println!("Uso: task <comando> <descrição>");
        return;
    }
    //Lendo os argumentos passados acessando indices do vetor (notar o & para acessar valor de ponteiros)
    let comando = &args[1];
    let description = &args[2];

    if comando == "add" {
        println!("{}", adicionar(&description));
    } else {
        println!("Comando não reconhecido tente task <add> <description>")
    }
}
