use std::time::Instant;
use std::thread;

fn main() {
    for _ in 1..10 {
        let start = Instant::now();

        let result = a(100000);

        println!("{} ms result={}", start.elapsed().as_millis(), result);
    }
}

fn a(n: i32) -> i32 {
    let mut result: i32 = 1;
    for i in 1..n {
        result = (result + i) * b(i);
    }
    return result;
}

fn b(n: i32) -> i32 {
    let mut result: i32 = 0;
    for i in 1..n {
        if result == 1_000_000 {
            thread::yield_now();
        }
        result += i * i;
    }
    return result;
}
