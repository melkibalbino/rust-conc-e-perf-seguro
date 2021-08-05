pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    fn local_sum(a: i32, b: i32) -> i32 {
        a + b
    }

    #[test]
    fn sum_test() {
        assert_eq!(local_sum(2, 2), sum(2, 2));
        assert_eq!(local_sum(8, 2), sum(8, 2));
    }

    #[test]
    #[ignore]
    fn subtract_test() {
        assert_eq!(0, subtract(2, 2));
        assert_eq!(6, subtract(8, 2));
    }

    #[test]
    #[ignore]
    fn multiply_test() {
        assert_eq!(4, multiply(2, 2));
        assert_eq!(16, multiply(8, 2));
    }

    #[test]
    #[ignore]
    fn divide_test() {
        assert_eq!(1, divide(2, 2));
        assert_eq!(4, divide(8, 2));
    }
}