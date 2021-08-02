fn main() {
    let mut a = String::from("Rust");

    a.reserve(50);

    a.push_str(" rules");
    println!("a: {} -> {}", a.capacity(), a);

    a.shrink_to_fit();
    println!("a: {} -> {}", a.capacity(), a);

    a.truncate(4);

    println!("a: {} -> {}", a.capacity(), a);
    a.shrink_to_fit();
    println!("a: {} -> {}", a.capacity(), a);
    a.clear();
    println!("a: {} -> {}", a.capacity(), a);

    // Remover caracter
    let mut rustlang = String::from("Rust");
    println!("Before remove: {}", rustlang);

    rustlang.remove(2);

    println!("After remove: {}", rustlang);
}
