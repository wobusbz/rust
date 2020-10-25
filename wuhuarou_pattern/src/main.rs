fn main() {
    let i = 1;
    match i {
        1 => println!("noe"),
        2 => println!("two"),
        _ => println!("xxx"),
    };

    println!("x = {}", i);

    let x = 'c';

    match x {
        'c'..='j' => println!("1"),
        'k'..='z' => println!("2"),
        _ => println!("other"),
    };
    println!("c = {:?}", i);
    println!("Hello, world!");
}
