fn main() {
    let name = String::from("rust");

    for chr in name.chars() {
        println!("Char: {}", chr);
    }

    println!("--------------------------");

    for bt in name.bytes() {
        println!("Byte: {}", bt);
    }

    println!("--------------------------");

    // iterar com indice
    for (idx, chr) in name.chars().enumerate() {
        println!("Indice: {}, Char: {}", idx, chr)
    }
}
