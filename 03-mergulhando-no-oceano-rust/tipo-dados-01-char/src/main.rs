fn main() {
    let a: char = '\u{2764}';
    let b: char = '9';
    let c: char = '0';

    println!("{}", a);
    println!("------- METODOS UTEIS -------");


    // metodos uteis
    println!("{} é um digito? {}", a, a.is_digit(10));
    println!("{} é binario? {}", a, a.is_digit(2));
    println!("------------------------------");

    println!("{} é um digito? {}", b, b.is_digit(10));
    println!("{} é binario? {}", b, b.is_digit(2));
    println!("------------------------------");

    println!("{} é um digito? {}", c, c.is_digit(10));
    println!("{} é binario? {}", c, c.is_digit(2));
    println!("------------------------------");

}
