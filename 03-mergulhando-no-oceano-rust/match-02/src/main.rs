fn check_number(num: i16) -> () {
    match num {
        0 => println!("Zero"),
        2 | 3 | 5 | 7 | 11 => println!("Prime"),
        _ => println!("Any number"),
    }
}

#[allow(unused_variables)] // permite variaveis nao usadas 
fn check_tuple(t: (i32, i32)) -> () {
    match t {
        (x, 0) => println!("Second is Zero"),// outra forma seria colocar '_' no começo do nome da variavel Ex: _x
        (0, x) => println!("First is Zero"),
        _ => println!("No Zeroes"),
    }
}

#[allow(dead_code)] // diretiva allow() permite | dead_code == codigo nao utilizado
enum Gender {
    Male,
    Female,
    Other,
}

fn main() {
    let number_a = 0;
    let number_b = 3;
    let number_c = 8;

    check_number(number_a);
    check_number(number_b);
    check_number(number_c);

    let gender: Gender = Gender::Male;

    match gender {
        Gender::Male => println!("Masculino"),
        Gender::Female => println!("Feminino"),
        Gender::Other => println!("Outro"),
    }

    check_tuple((0, 10));
    check_tuple((33, 0));
    check_tuple((8, 12))
}
