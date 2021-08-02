fn main() {
    let editora = "Casa do Codigo".to_string();
    let autor = String::from("Marcelo Castellani");
    let livro: String = "A linguagem Rust".into();

    let mut setence = String::from("Este é o livro ");
    setence += &livro;
    setence += " da ";
    setence += &editora;
    setence += " e escrito por ";
    setence += &autor;
    setence += ".";

    println!("{}", setence)
}
