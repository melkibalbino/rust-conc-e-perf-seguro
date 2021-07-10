fn sum(num1: i32, num2: i32) -> i32 {
    // return 0; // return para retorna o valor antes do final da função
    num1 + num2 // nessa caso nao e nescessario usar return
}

fn main() {
    let num1: i32 = 10;
    let num2: i32 = 25;

    println!("{} + {} = {}", num1, num2, sum(num1, num2))
}
