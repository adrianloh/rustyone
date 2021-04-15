use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::thread;

use crossbeam::channel::{self, Receiver, Sender};
use ureq;

#[derive(Debug)]
struct Doug(String, u64);

const THREADS: usize = 8;
const ZEROLEN: u64 = 0;

fn main() {
    // The pipe looks like this:
    //  get --> getter --> collect --> collector -|
    //            ^---------------------reget <---|
    let (get, getter) = channel::unbounded::<String>();
    let (collect, collector) = channel::bounded::<Doug>(1);
    let reget = get.clone(); // Retries are sent here

    let producer = thread::spawn(move || read_urls(get));

    // Spawn worker threads
    // A worker thread receives a url from `getter`, calls HEAD on the url and
    // sends its result to the `collect` channel for collection
    for _ in 1..=THREADS {
        let getter_clone = getter.clone();
        let collect_clone = collect.clone();
        thread::spawn(move || get_head(getter_clone, collect_clone));
    }

    // We need the total number of tasks from the producer so we'll know when to break
    // Since the `collect` channel is bounded, senders are blocked until the collector
    // starts receiving
    let total_dougs = producer.join().unwrap();
    let mut dougs = vec![];
    println!("total: {}", total_dougs);

    'collector: loop {
        let doug = collector.recv().unwrap();
        if doug.1 == ZEROLEN {
            // If something failed, send it back to retry
            reget.send(doug.0).unwrap();
        } else {
            dougs.push(doug);
        }
        if dougs.len() == total_dougs {
            break 'collector;
        }
    }
}

// Get urls from file and send them into the `get` channel.
// Return the number of urls
// When this function returns and `get` is dropped, this
// essentially closes the channel and any receiving loop
// terminates once the channel is drained
fn read_urls(get: Sender<String>) -> usize {
    let file = File::open("testfiles/100rows.csv").unwrap();
    BufReader::new(file)
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let cols: Vec<&str> = line.split(",").collect();
            let url = format!("https://avx.ttwp.xyz/Avaxgirls-20{}", cols[0].to_owned());
            get.send(url).unwrap()
        })
        .count()
}

fn get_head(getter: Receiver<String>, collect: Sender<Doug>) {
    let agent = ureq::agent();
    for url in getter.iter() {
        let size = match agent.head(&url).call() {
            Err(_) => ZEROLEN,
            Ok(response) => {
                let s = response.header("Content-Length").unwrap();
                u64::from_str(s).unwrap()
            }
        };
        let doug = Doug(url, size);
        println!("{:?}", &doug);
        collect.send(doug).unwrap();
    }
}
