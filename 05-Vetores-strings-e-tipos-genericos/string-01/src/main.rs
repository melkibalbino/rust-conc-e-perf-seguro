fn main() {
    let editora = "Casa do Codigo".to_string();
    let autor = String::from("Marcelo Castellani");
    let livro: String = "A linguagem Rust".into();
    println!("Este é o livro \"{}\" da editora \"{}\"\nEscrito por \"{}\"", livro, editora, autor);
}
