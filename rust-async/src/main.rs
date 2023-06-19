use std::time::Instant;

const THRESHOLD: u32 = 1_000;

#[tokio::main]
async fn main() {
    for _ in 1..10 {
        let start = Instant::now();

        let result = a(100_000_000).await;

        println!("{} ms result={}", start.elapsed().as_millis(), result);
    }
}

async fn a(n: u32) -> u32 {
    let mut result: u32 = n;
    for _ in 0..n {
        result = b(result).await;
    }
    return result;
}

async fn b(n: u32) -> u32 {
    let mut result: u32 = n;

    result = result * 1_999_999_981;
    if result < THRESHOLD {
        tokio::task::yield_now().await;
    }

    result = result * 1_999_999_981;
    if result < THRESHOLD {
        tokio::task::yield_now().await;
    }

    result = result * 1_999_999_981;
    if result < THRESHOLD {
        tokio::task::yield_now().await;
    }

    return result;
}
