use std::sync::Mutex;

fn main() {

    let me = Mutex::new(5);
    {
        let mut num = me.lock().unwrap();
        *num = 6;
    }

    println!("me = {:?}", me);
    println!("Hello, world!");
}
