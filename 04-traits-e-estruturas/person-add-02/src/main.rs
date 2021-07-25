use std::ops::Add;

#[derive(Copy, Clone, Debug)]
struct Person {
    name: &'static str,
    age: i32
}

impl Add<i32> for Person {
    type Output = Person;

    fn add(self, b: i32) -> Person {
        Person { name: self.name, age: self.age + b }
    }
}

fn main() {
    let p1 = Person { name: "Marcelo", age: 39 };
    let x = p1 + 10;

    println!("{:?}\n", p1);
    println!("{:?}", x)
}
