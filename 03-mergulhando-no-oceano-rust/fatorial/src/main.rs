fn fatorial(valor: u32) -> u32 {
    if valor == 0 {1} else {valor * fatorial(valor - 1)}
}

fn main() {
    let numero: u32 = 6;
    println!("Fatorial de {} é {}.", numero, fatorial(numero))
}
