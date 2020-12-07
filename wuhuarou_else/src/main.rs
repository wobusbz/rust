fn main() {
    println!("Hello, world!");

    let a = [1, 2, 3];

    let b = &a;
    println!("{:p}", b);

    let mut c = vec![1, 2, 3, 4];
    c.push(5);
    println!("{:?}", c);
    println!("{:?}", c.pop());
    let e = &42;
    assert_eq!(42, *e);

    assert_eq!(fizz_buzz(15), "fizz_buzz".to_string());
    assert_eq!(fizz_buzz(3), "fizz".to_string());
    assert_eq!(fizz_buzz(5), "buzz".to_string());
    assert_eq!(fizz_buzz(13), "13".to_string());

    assert_eq!(math(sum, 1, 2), 3);
    assert_eq!(math(sub, 1, 2), -1);

    let mut arr = [0; init_len()];
    for x in 0..5 {
        arr[x] = x;
    }
    println!("{:?}", arr);

    let close_anntated = |i: i32, j: i32| -> i32 { i + j };
    let close_inferred = |i, j| i + j;

    let i = 1;
    let j = 6;
    assert_eq!(close_anntated(i, j), 7);
    assert_eq!(close_inferred(i, j), 7);
    let x = 5;
    let y = 7;
    assert_eq!(close_math(|| x + y), 12);
    assert_eq!(close_math(|| x * y), 35);
    let result = two_times_impl();
    assert_eq!(result(2), 4);

    let isBool = if 1 > 2 { true } else { false };
    println!("{}", isBool);
}

pub fn fizz_buzz(num: i32) -> String {
    if num % 15 == 0 {
        return "fizz_buzz".to_string();
    } else if num % 3 == 0 {
        return "fizz".to_string();
    } else if num % 5 == 0 {
        return "buzz".to_string();
    } else {
        return num.to_string();
    }
}

pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

const fn init_len() -> usize {
    return 5;
}

fn close_math<F: Fn() -> i32>(op: F) -> i32 {
    op()
}

fn two_times_impl() -> impl Fn(i32) -> i32 {
    let i = 2;
    move |j| j * i
}
