use std::ops::Add;

#[derive(Copy, Clone)]
struct Person {
    name: &'static str,
    age: i32
}

impl Add<i32> for Person {
    type Output = i32;

    fn add(self, b: i32) -> i32 {
        self.age + b
    }
}

fn main() {
    let p1 = Person { name: "Marcelo", age: 39 };
    let x = p1 + 10;

    println!("A nova idade de {} é {}", p1.name, x)
}
