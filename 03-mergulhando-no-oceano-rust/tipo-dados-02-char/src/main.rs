fn main() {
    let a: char = '→';

    // Retornar representação em uma string
    let repr: String = a.escape_unicode().collect();
    println!("{} é representado por {}", a, repr)
}
