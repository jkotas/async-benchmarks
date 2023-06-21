use std::time::Instant;
use std::thread;

const THRESHOLD: u32 = 20_000_000;

fn main() {
    for _ in 1..10 {
        let start = Instant::now();

        let result = a(100_000_000);

        println!("{} ms result={}", start.elapsed().as_millis(), result);
    }
}

fn a(n: u32) -> u32 {
    let mut result: u32 = n;
    for _ in 0..n {
        result = b(result);
    }
    return result;
}

fn b(n: u32) -> u32 {
    let mut result: u32 = n;

    result = result * 1_999_999_981;
    if result < THRESHOLD {
        thread::yield_now();
    }

    result = result * 1_999_999_981;
    if result < THRESHOLD {
        thread::yield_now();
    }

    result = result * 1_999_999_981;
    if result < THRESHOLD {
        thread::yield_now();
    }

    return result;
}
