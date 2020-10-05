use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals =vec![String::from("hello"), String::from("wuhuarou")];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
    });

    
    thread::spawn(move || {
        let vals =vec![String::from("hello fack"), String::from("wuhuarou")];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
    });


    for rec in rx {
        println!("Got: {}", rec);
        // thread::sleep(Duration::from_secs(1));
    }
    println!("Hello, world!");
}
