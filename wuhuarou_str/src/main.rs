fn main() {

    let s = "hello world".to_string();

    println!("{}", s);


    let mut s1 = String::new();
    s1.push_str("hello world");
    println!("{}", s1);
    println!("Hello, world!");
}
