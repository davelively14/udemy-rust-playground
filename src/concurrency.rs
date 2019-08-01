use std::{thread, time};

pub fn run_examples() {
    let handle = thread::spawn(|| {
        println!("From a thread!");
    });

    thread::sleep(time::Duration::from_millis(1_000));

    println!("In main thread");

    handle.join();
}
