use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{} in spawn", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("{} in main", i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("Hello, world!");
    handle.join().unwrap();
}
