use std::fmt::Display;
use std::time::Instant;
use tokio::spawn;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let mut now = Instant::now();

    // Blocks for a second
    let x = ten_plus_(10).await;
    assert_eq!(x, 20);
    report("slept", now.elapsed().as_secs_f64());

    now = Instant::now();

    // Spawn concurrent tasks
    let mut handles = vec![];
    for mut k in 1..=10_000 {
        // If we're just calling `ten_plus_()`, we can just do:
        // `spawn(ten_plus_(x))`
        // `spawn()` does the awaiting for us
        let join_handle /*tokio::task::JoinHandle*/ =
            spawn(async move {
                k = ten_plus_(k).await;
                ten_plus_(k).await
            });
        handles.push(join_handle);
    }
    report("spawned", handles.len());

    // Join tasks
    let mut sum = 0;
    for handle in handles {
        let x = handle.await.unwrap();
        sum += x;
    }
    report("sum", sum);

    // Since the tasks ran concurrently, we've
    // blocked for only ~2 seconds
    report("elapsed", now.elapsed().as_secs_f64());
}

fn report(col: &str, n: impl Display) {
    println!("{:>10}: {:.4}", col, n);
}

async fn ten_plus_(x: u64) -> u64 {
    sleep(Duration::from_millis(1000)).await;
    10 + x
}
