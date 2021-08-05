extern crate calculator_01;

#[test]
fn sum_test() {
    assert_eq!(4, calculator_01::sum(2, 2));
    assert_eq!(10, calculator_01::sum(8, 2));
}

#[test]
fn subtract_test() {
    assert_eq!(0, calculator_01::subtract(2, 2));
    assert_eq!(6, calculator_01::subtract(8, 2));
}

#[test]
fn multiply_test() {
    assert_eq!(4, calculator_01::multiply(2, 2));
    assert_eq!(16, calculator_01::multiply(8, 2));
}

#[test]
fn divide_test() {
    assert_eq!(1, calculator_01::divide(2, 2));
    assert_eq!(4, calculator_01::divide(8, 2));
}
