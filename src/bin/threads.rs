use std::thread;
use std::time::Duration;

use rand::Rng;

struct O {
    number: u32
}

fn main() {
    let mut handlers = Vec::new();
    for i in 1..=10 {
        let o = O { number: i };
        handlers.push(thread::spawn(move || work(o)));
    }
    let mut total = 0;
    for handler in handlers {
        let result = handler.join().unwrap_or(0);
        if result > 0 {
            println!("<== {}", &result);
            total += result;
        }
    }
    println!("total: {}", total);
}

fn work(o: O) -> u32 {
    println!("==> {}", o.number);
    let delay = (o.number as u64) * 500;
    thread::sleep(Duration::from_millis(delay));
    if rand::thread_rng().gen_bool(0.2) {
        panic!("oh crap!")
    }
    o.number
}
