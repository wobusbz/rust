fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_t<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let number_list = vec![1, 2, 37, 6, 3];
    println!("{}", largest_i32(&number_list));
    println!("{}", largest_t(&number_list));
    let integer = Point { x: 1, y: 2 };

    println!("{:?}", integer);
    println!("Hello, world!");
}
