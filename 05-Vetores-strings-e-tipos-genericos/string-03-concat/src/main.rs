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

    println!("{}", setence);

    let mut setence2 = String::from("Este é o livro ");
    setence2.push_str(&livro);
    setence2.push_str(" da ");
    setence2.push_str(&editora);
    setence2.push_str(" e escrito por ");
    setence2.push_str(&autor);
    setence2.push_str(".");

    println!("-----------------------");
    println!("{}",setence2);

    // Usando Char para Contatenar
    let mut sentence3 = String::from("Hey ");
    sentence3.push('Y');
    sentence3.push('a');
    sentence3.push('a');
    sentence3.push('h');
    sentence3.push('!');
    println!("-----------------------");
    println!("{}", sentence3)
}
