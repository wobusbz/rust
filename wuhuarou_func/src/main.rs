
fn ofther_func(i: u32)  {
    println!("{}", i);
}

fn other_func1(i: u32, j: u32) -> u32 {
    return i + j;
}

fn main() {
    ofther_func(2);
    println!("Hello, world!");
    let i: u32 = 1;
    let j: u32 = 2;
    let sum = other_func1(i,j);
    println!("{}", sum);
}
