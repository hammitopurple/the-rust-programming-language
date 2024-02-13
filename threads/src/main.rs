use std::thread;
use std::time::Duration;

use std::sync::mpsc; // multi producer, single consumer

fn simple_thread_join() {
    // Assign the return value to the variable `handle`
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // Call join method such that the main process will wait for the thread to finish
    handle.join().unwrap();

    // Using `move` to take ownership of the vector values
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn using_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello!");
        tx.send(val).unwrap()
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

fn sending_multiple_values() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Possible to treat rx as an iterator
    for received in rx {
        println!("Got: {received}");
    }
}

fn multiple_transmitters() {
    let (tx, rx) = mpsc::channel();

    // Clone tx and use it to send messages
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

    // Send messages using the original tx
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

    // From the output, you can see that the sequence of receiving the messages
    // from both threads are not deterministic
    for received in rx {
        println!("Got: {}", received);
    }
}

fn main() {
    simple_thread_join();
    using_channel();
    sending_multiple_values();
    multiple_transmitters();
}
