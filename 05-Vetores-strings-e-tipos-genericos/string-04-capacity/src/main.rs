fn main() {
    let a = String::with_capacity(255);
    let b = String::from("ABC");
    let mut c = String::with_capacity(255);
    c += "Jamaica"; // Apesar de receber um valor com tamanho menor a capacidade continua a mesma


    println!("a: {} -> {} -> \"{}\"", a.len(), a.capacity(), a);
    println!("b: {} -> {} -> \"{}\"", b.len(), b.capacity(), b);
    println!("c: {} -> {} -> \"{}\"", c.len(), c.capacity(), c);

    // Reservar espaço em uma variavel ja criada
    let mut nome = String::from("Jurema");
    println!("nome: {} -> {} -> \"{}\"", nome.len(), nome.capacity(), nome);

    nome.reserve(50);

    println!("nome: {} -> {} -> \"{}\"", nome.len(), nome.capacity(), nome);

    // Remove Espaço adicional
    nome.shrink_to_fit();

    println!("nome: {} -> {} -> \"{}\"", nome.len(), nome.capacity(), nome)

}
