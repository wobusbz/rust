fn main() {
    println!("Hello, world!");
    println!("{}", String::from("hello world"));

    let s = String::from("s: &mut str");
    println!("{}", s);

    let mut v = vec![];

    for i in 0..10{
        v.push(i);
    }
    v.insert(10, 11);
    println!("{:?}", v);
}
