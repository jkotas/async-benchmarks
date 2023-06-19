use std::time::Instant;

#[tokio::main]
async fn main() {
    for _ in 1..10 {
        let start = Instant::now();

        let result = a(100000).await;

        println!("{} ms result={}", start.elapsed().as_millis(), result);
    }
}

async fn a(n: i32) -> i32 {
    let mut result: i32 = 1;
    for i in 1..n {
        result = (result + i) * b(i).await;
    }
    return result;
}

async fn b(n: i32) -> i32 {
    let mut result: i32 = 0;
    for i in 1..n {
        if result == 1_000_000 {
             tokio::task::yield_now().await;
        }
        result += i * i;
    }
    return result;
}
