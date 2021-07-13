fn check_grade(grade: f32) -> () {
    if grade >= 0.0 && grade < 4.9 {
        println!("Disapproved");
    } else if grade >= 4.9 && grade < 6.0 {
        println!("Exam");
    } else if grade >= 6.0 {
        println!("Aprroved");
    } else {
        println!("Invalid grade!!!")
    }
}

fn main() {
    // Condicionando iniciação de uma variavel
    let x = if 10 + 5 == 15 {
        "10 + 5 é igual a 15"
    } else {
        "10 + 5 não é igual a 15"
    };
    println!("{}", x);

    let grade_a  = 0.0;
    let grade_b  = 3.2;
    let grade_c  = 5.1;
    let grade_d  = 8.3;

    check_grade(grade_a);
    check_grade(grade_b);
    check_grade(grade_c);
    check_grade(grade_d)
}
