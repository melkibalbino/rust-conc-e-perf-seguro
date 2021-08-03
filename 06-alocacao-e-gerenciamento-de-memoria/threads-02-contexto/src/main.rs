use std::thread;

fn main() {
    let mut value = 10;

    let a = thread::spawn(move || {
        value = value + 123;
        println!("Thread A: {}", value);
    });

    let b = thread::spawn(move || {
        value = value + 1;
        println!("Thread B: {}", value);
    });

    let _ = a.join();
    let _ = b.join();

    println!("Main thread {}", value);
}
