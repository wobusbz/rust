fn main() {
    let use_closure = || {
        println!("hello world");
    };
    use_closure();
    println!("Hello, world!");

    // 闭包会为每个函数值和返回值类型推到一个具体的类型，但是不能推导两次
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;
    println!("{}", add_one_v1(1));
    println!("{}", add_one_v2(2));
    println!("{}", add_one_v3(3));
    println!("{}", add_one_v4(4));

    let i = 1;
    let exe = |x| x + 1;
    let r3 = exe(5);
    println!("{}", r3);
}

fn add_one_v1(x: u32) -> u32 {
    x + 1
}
