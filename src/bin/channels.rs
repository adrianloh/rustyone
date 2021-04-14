use std::thread;
use std::time::Duration;

use crossbeam::channel;
use rand::Rng;

#[derive(Debug)]
struct O {
    number: u32,
    sleep: u64,
}

fn main() {
    let mut random = rand::thread_rng();

    let (omake, omaker) = channel::bounded::<u32>(1);
    let (odone, odoner) = channel::bounded::<O>(1);

    // Spawn loop
    let mut workers = vec![];
    for thread_n in 1..=8 {
        let maker = omaker.clone();
        let done = odone.clone();
        let sleep: u64 = random.gen_range(250..2000);
        let worker = thread::spawn(move || {
            for number in maker.iter() {
                // This loop breaks when `omake` is dropped and the channel is drained
                thread::sleep(Duration::from_millis(sleep));
                done.send(O { number, sleep }).unwrap();
            }
            thread_n
        });
        workers.push(worker)
    }

    // Initial producer
    thread::spawn(move || {
        for i in 1..=50 {
            omake.send(i).unwrap();
        }
        drop(omake); // This is implied, but we're being explicit here
    });

    // Join workers
    thread::spawn(move || {
        for worker in workers {
            let i = worker.join().unwrap();
            println!("thread done: {}", i)
        }
        drop(odone); // Close the channel so collect() finishes
    });

    // Collect
    let ooo: Vec<O> = odoner.iter()
        .inspect(|o| println!("{:?}", o))
        .collect();
    println!("total collected: {}", ooo.len());
}

