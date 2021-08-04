fn even_test(number: u32) -> Option<u32> {
    if number % 2 == 0 {
        Some(number)
    } else {
        None
    }
}

fn main() {
    println!("This number is even: {}", even_test(22).unwrap());

    // Quando o retorno do Option é None e é esperado um valor
    // Causa um erro
    println!("This number is even: {}", even_test(15).unwrap());
}
