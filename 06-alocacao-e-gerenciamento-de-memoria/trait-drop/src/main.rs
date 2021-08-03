struct RaceCar {
    number: i32
}

impl Drop for RaceCar {
    fn drop(&mut self) {
        println!("Car {} finished the run", self.number);
    }
}

#[allow(unused_variables)]
fn main() {
    let car_a = RaceCar { number: 3 };
    let car_b = RaceCar { number: 5 };
    let car_c = RaceCar { number: 8 };
}
