use fibora::{fibonacci, fibonacci_sequence};

fn main() {
    for i in 0..10 {
        println!("{}", fibonacci(i));
    }

    println!("{:?}", fibonacci_sequence(10));
}
