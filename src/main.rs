use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main()  {
    let (tx, rx) = mpsc::channel();

    // We can have multiple producers & a single consumer (mpsc), so we can clone a sender & use it in another thread
    // Each thread gets their own producer/sender/transmitter
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    /*
        Different outputs happen depending on the system
        We can get different/weirder results by experimenting w/ thread::sleep() as well
    */
}