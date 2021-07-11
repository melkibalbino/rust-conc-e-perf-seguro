enum Gender {
    Female,
    Male,
    Other
}

struct Person {
    name: &'static str,
    gender: Gender
}

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
