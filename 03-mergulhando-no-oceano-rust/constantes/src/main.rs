fn main() {
    const Y: i32 = 3; // Com constantes a inferencia de tipo nao se aplica
    // ou seja, deve-se declarar o tipo
    println!("Const: {}", Y);
}
