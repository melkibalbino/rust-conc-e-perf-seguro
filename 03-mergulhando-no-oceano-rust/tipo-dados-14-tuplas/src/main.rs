fn main() {
    let a = (1, "Teste", 'c');
    let a0 = a.0;
    let a1 = a.1;
    let a2 = a.2;
    println!("{}, {} e {}", a0, a1, a2);

    // let destrutivo
    let (d0, d1, d2) = a;
    println!("{}, {} e {}", d0, d1, d2);

    let t1 = (1, "Um");
    let t2 = (2, "Dois");
    let t3 = (3, "Tres");

    let ta = [t1, t2, t3];
    println!("{:?}", ta)


    // Retorna um erro pois o formato das tuplas estão diferentes
//     let a = (1, 45.44);
//     let b = ("Numero", "dois");
//     let c = ("Letra", 'c');
//     let d = [a, b, c];

}
