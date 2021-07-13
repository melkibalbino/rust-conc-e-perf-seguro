#[allow(dead_code)]
enum Gender {
    Female,
    Male,
    Other
}

#[allow(dead_code)]
struct Person {
    name: &'static str,
    gender: Gender
}

#[allow(unused_variables)]
fn main() {
    let nelson = Person {
        name: "Nelson Castellani",
        gender: Gender::Male
    };

    let adelia = Person {
        name: "Adelia Maria Fontes",
        gender: Gender::Female
    };
}
