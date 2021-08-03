fn main() {
    let a: i32 = 8;
    let b: i8 = a as i8;
    println!("a: {}", a);
    println!("b: {}", b);

    println!("------------------------");
    let x: i32 = 2055;
    let y: i16 = x as i16;
    let z: i8 = x as i8;

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z)
    // Nosso 2055 virou 7 . Isso porque, durante a conversão, os
    // bits excedentes foram desprezados. Veja a seguir.
    // Ex:
    // 2055 -> 0000100000000111
    // 7    ->         00000111
}
