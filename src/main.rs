// Programa para treinar CLI's in Rust
// task add <description>
// task list <all/todo/done/in-progress>
// task delete <id>
// task update <id> <description>
// task mark-done <id>
// task mark-in-progress <id>

fn adicionar(desc: &str) -> String {
    format!("Tarefa \"{desc}\" adicionada com sucesso")
}

fn main() {
    println!("====================== GERENCIE SUAS TAREFAS =============================\n\n\n\n");
    //Pegar os argumentos passados para o programa 'argv[i]'

    let task = adicionar(&String::from("Estudar Rust"));
    println!("{task}");
}
