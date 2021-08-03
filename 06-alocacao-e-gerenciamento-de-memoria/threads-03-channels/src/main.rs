use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static CARS: i32 = 5;

fn main() {
    let (sender, receiver): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..CARS {
        let thread_sender = sender.clone();

        thread::spawn(move || {
            let _ = thread_sender.send(id);

            println!("Car {} finished the race", id);
        });
    }

    let mut ids = Vec::with_capacity(CARS as usize);

    for _ in 0..CARS {
        ids.push(receiver.recv());
    }

    println!("Final order -> {:?}", ids)
}
