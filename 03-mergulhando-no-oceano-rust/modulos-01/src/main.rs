mod compute {
    // Private function
    fn is_zero(number: i32) -> bool {
        // if number == 0 { return true }
        // false
        number == 0
    }

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn divide(a: i32, b: i32) -> i32 {
        if is_zero(b) {
            return 0;
        }
        a / b
    }
}

// Criando apelido para função no modulo
use compute::add as my_add;

fn main() {
    let num1: i32 = 10;
    let num2: i32 = 4;

    println!("{} + {} = {}", num1, num2, my_add(num1, num2)); // não precisa chamar o compute:: nesse caso
    println!("{} - {} = {}", num1, num2, compute::subtract(num1, num2));
    println!("{} x {} = {}", num1, num2, compute::multiply(num1, num2));
    println!("{} / {} = {}", num1, num2, compute::divide(num1, num2));
    println!("{} / 0 = {}", num1, compute::divide(num1, 0));
}
