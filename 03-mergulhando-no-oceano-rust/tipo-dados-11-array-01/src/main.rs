fn main() {
    let a = ["Marcelo", "Rust", "Castellani"];
    println!("Programando em {}, por {} {}.", 
            a[1], a[0], a[2]);

    // [T; N] T igual a Tipo ou Valor e N a quantidade de itens
    let b = [0; 5]; // é igual a [0, 0, 0, 0, 0]
    println!("Programando em {}, por {} {}.", 
            b[1], b[0], b[2]);

    // Quantidades de elementos
    println!("Qtde: {}", b.len());

    // Retorna um erro ao tentar colocar outro tipo de valor no array
    // let mut c = [0; 5];
    // c[1] = "Rust";

    let mut d = [""; 5];
    d[1] = "Rust";
    d[0] = "Melki";
    d[2] = "Balbino";
    println!("Programando em {}, por {} {}.", 
            d[1], d[0], d[2]);
}
