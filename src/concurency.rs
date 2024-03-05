use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn using_move() {
    let v = vec![1, 2, 3, 4];

    thread::spawn(move || println!("{:?}", v));
}

fn main() {
    threads_rust();
    shared_state_concurrency();

    
}

fn shared_state_concurrency() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Results: {}", *counter.lock().unwrap());
}

fn threads_rust() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    using_move();

    let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    // let val = String::from("Hello!");

    // tx.send(val).unwrap();
    // });

    // let recived = rx.recv().unwrap();
    // println!("Got: {}", recived);

    handle.join().unwrap();

    // Sending muliptle messages in chanell

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("From"),
            String::from("The"),
            String::from("Other"),
            String::from("Thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(2));
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
        println!("{received}");
    }
}
