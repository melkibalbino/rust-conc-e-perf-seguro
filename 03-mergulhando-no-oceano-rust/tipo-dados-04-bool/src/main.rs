fn main() {
    let x = false;
    let y: bool = true;
    if x {
        println!("X is true!");
    }
    if y {
        println!("Y is true!")
    }
    
    // O rust espera que no if seja passado um booleano
    // diferente de outras linguagens, ele nao aceita numeros ou outros valores

    // let a = 1;
    // if a {
    //     println!("A é igual a 1")
    // }

    println!("==========================");
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true)
}
