use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![String::from("hello"), String::from("wuhuarou"), String::from("woaini")];

        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
    });

    for recv in rx {
        println!("Got: {}", recv);
    }
    println!("Hello, world!");
}
