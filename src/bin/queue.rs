use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;

use crossbeam::channel;
use crossbeam::channel::Receiver;
use crossbeam::queue::SegQueue;

#[derive(Debug)]
struct Image {
    url: String,
    height: u32,
    width: u32,
}

fn main() {
    const THREADS: u32 = 8;
    static Q1: SegQueue<String> = SegQueue::new();
    static Q2: SegQueue<Image> = SegQueue::new();
    let (close, closer) = channel::bounded::<bool>(1);

    let producer = thread::spawn(|| {
        // Process a stream of unknown length and push to the queue
        let file = File::open("testfiles/manymanyrows.csv").unwrap();
        let lines = BufReader::new(file).lines();
        lines.map(|line| Q1.push(line.unwrap())).count()
    });

    // Workers
    for i in 0..THREADS {
        let ch = closer.clone();
        thread::spawn(move || {
            process(&Q1, &Q2, ch);
            println!("thread done: {}", i)
        });
    }

    // Wait for producer to be done so we can get total
    let total = producer.join().unwrap();
    let mut images = vec![];
    println!("total: {}", total);

    'collector: loop {
        match Q2.pop() {
            Some(o) => images.push(o),
            None => continue,
        }
        if images.len() == total {
            // shutdown threads
            (0..THREADS).for_each(|_| close.send(true).unwrap());
            break 'collector;
        }
    }
    println!("collected: {}", images.len());
}

fn process(q1: &SegQueue<String>, q2: &SegQueue<Image>, kill: Receiver<bool>) {
    loop {
        //  `recv()` blocks and waits on empty channels (not what we want)
        //  `try_recv()` is non-blocking, and returns Err if channel is empty
        if kill.try_recv().unwrap_or(false) {
            break;
        }
        let line = match q1.pop() {
            Some(line) => line,
            None => continue,
        };
        let cols: Vec<&str> = line.split(',').collect();
        let url = cols[0].to_owned();
        let width = cols[1].parse::<u32>().unwrap();
        let height = cols[2].parse::<u32>().unwrap();
        let img = Image { url, width, height };
        //println!("{:?}", &img);
        q2.push(img);
    }
}
