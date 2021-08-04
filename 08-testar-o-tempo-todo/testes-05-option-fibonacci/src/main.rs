struct Fibonacci {
    curr: u32,
    next: u32
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // Fibonacci é infinito, mas para fins
        // didáticos o nosso irá até 21
        if self.curr > 21 {
            None
        } else {
            Some(self.curr)
        }
    }
}

fn main() {
    let fib1 = Fibonacci { curr: 1, next: 1 };
    println!("The first 4 terms of the Fibonacci sequence are: ");

    for i in fib1.take(4) {
        println!("> {}", i);
    }

    let fib2 = Fibonacci { curr: 1, next: 1 };
    println!("The next 4 terms of the Fibonacci sequence are: ");
    for i in fib2.skip(4).take(4) {
        println!("> {}", i)
    }
}
