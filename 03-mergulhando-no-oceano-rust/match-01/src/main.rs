#[allow(illegal_floating_point_literal_pattern)]
fn check_grade(grade: f32) -> () {
    // uso de rage float no match esta depreciado
    // porem foi colocado para seguir o exemplo do livro
    // para nao retornar erro foi colocado o decorador permitindo
    // #[allow(illegal_floating_point_literal_pattern)]
    match grade {
        0.0..=4.8  => println!("Disapproved"),
        4.9..=5.9  => println!("Exam"),
        5.9..=10.0 => println!("Approved"),
        _ => println!("Invalid grade!!"),
    }
}

fn main() {

    let grade_a  = 0.0;
    let grade_b  = 3.2;
    let grade_c  = 5.1;
    let grade_d  = 8.3;
    let grade_e  = 19.8;

    check_grade(grade_a);
    check_grade(grade_b);
    check_grade(grade_c);
    check_grade(grade_d);
    check_grade(grade_e)
}
