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

    let points = Points { x: 1, y: 2 };
    println!("{}", points.get_x());
    println!("{}", points.get_y());

    let p1 = Points2 { x: 5, y: 1.1 };
    let p2 = Points2 { x: "hello", y: 'C' };
    let p3 = p1.create_point(p2);
    println!("p3.X = {}, p3.y = {}", p3.x, p3.y);
    println!("Hello, world!");
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct Points<T> {
    x: T,
    y: T,
}

impl<T> Points<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}

struct Points2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Points2<T, U> {
    fn create_point<V, W>(self, other: Points2<V, W>) -> Points2<T, W> {
        Points2 {
            x: self.x,
            y: other.y,
        }
    }
}
