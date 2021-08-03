extern crate num;

fn generic_sum<T: num::Num>(x: T, y: T) -> T {
    x.add(y)
}

fn main() {
    println!("{}", generic_sum::<i32>(1, 2));
    println!("{}", generic_sum::<i16>(3, 4));
    println!("{}", generic_sum::<i8>(5, 6));
    println!("{}", generic_sum::<f32>(1.3, 2.24));
}
