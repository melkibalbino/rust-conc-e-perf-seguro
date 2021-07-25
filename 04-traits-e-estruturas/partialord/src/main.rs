use std::cmp::Ordering;

#[derive(PartialOrd, PartialEq, Eq, Clone, Copy)]
struct Person {
    age: i32,
    name: &'static str
}

impl Ord for Person {
    fn cmp(&self, other: &Person) -> Ordering{
        (self.age).cmp(&(other.age))
    }
}

fn order(p1: Person, p2: Person) -> () {
    if p1 > p2 {
        println!("{} é mais velha que {}.", p1.name, p2.name)
    } else {
        println!("{} é mais nova que {}.", p1.name, p2.name)
    }
}

fn main() {
    let sivirina = Person {
        age: 55,
        name: "Sivirina ChiqueChique"
    };

    let jurema = Person {
        age: 31,
        name: "Jurema CalangoSeco"
    };

    let amarelo = Person {
        age: 15,
        name: "Minino Amarelo"
    };

    order(jurema, sivirina);
    println!("-----------------------");
    order(sivirina, amarelo)
}
