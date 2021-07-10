fn main() {
    println!("{}", 'a'.is_alphabetic());
    println!("{}", ' '.is_alphabetic());
    println!("{}", '→'.is_alphabetic());
    println!("=======================");

    println!("Uppercase -> {}", 'a'.is_uppercase());
    println!("Lowercase -> {}", 'a'.is_lowercase());
    println!("Whitespace -> {}", 'a'.is_whitespace());
    println!("Alphanumeric -> {}", 'a'.is_alphanumeric());
    println!("Numeric -> {}", 'a'.is_numeric());

    println!("{}", 'a'.is_digit(16)) // Hexadecimal
}
