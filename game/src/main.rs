use std::io;

fn main() {
    let mut guess =  String::new();    
    io::stdin().read_line(&mut guess).expect("hello world");
    println!("You guessed:{}", guess);
    println!()
}
    