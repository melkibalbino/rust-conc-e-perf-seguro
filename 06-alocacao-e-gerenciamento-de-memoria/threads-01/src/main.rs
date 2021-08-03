use std::thread;

fn main() {
    let a = thread::spawn(|| {
        println!("Hello Thread a!");
    });

    let b = thread::spawn(|| {
        println!("Hello Thread b!");
    });

    let _ = a.join();
    let _ = b.join();
}
