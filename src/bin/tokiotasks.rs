use tokio::spawn;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handles: Vec<_> = (1..=10000)
        .map(|x| {
            spawn(ten_plus_(x)) //
        })
        .collect();
    println!("spawned: {}", handles.len());
    let mut sum = 0;
    for handle in handles {
        let x = handle.await.unwrap();
        sum += x;
    }
    println!("sum: {}", sum);
}

async fn ten_plus_(x: i32) -> i32 {
    sleep(Duration::from_millis(2000)).await;
    10 + x
}
