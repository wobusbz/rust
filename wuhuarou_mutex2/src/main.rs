use std::sync::Mutex;
use std::thread;
use std::sync::Arc;

fn main() {

    let me = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..10 {
        let new_me = Arc::clone(&me);
        let handle = thread::spawn(move || {
            let mut num = new_me.lock().unwrap();
            *num +=1
        });
        handlers.push(handle);
    }

    for handler in handlers {
        handler.join().unwrap();
    }
    println!("result = {}", *me.lock().unwrap());
    println!("Hello, world!");
}
