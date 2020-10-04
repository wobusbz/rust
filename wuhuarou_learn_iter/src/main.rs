fn main() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);
    }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);
    }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);
    }

    println!("{:?}", v1);
    // ----------迭代器可变引用------------
    let mut v2 = vec![5, 2, 0];
    let mut v2_iter = v2.iter_mut();
    if let Some(v) = v2_iter.next() {
        *v = 3
    }
    println!("{:?}", v2);

    //----------消费适配器----------------
    let v3 = vec![1, 2, 3];
    let v3_iter = v3.iter();
    let total: i32 = v3_iter.sum(); // 调用消费适配器求和sum
    println!("{}", total);

    //---------迭代适配器----------------
    let v4 = vec![1, 2, 3];
    let v5: Vec<_> = v4.iter().map(|x| x + 1).collect();
    println!("v2 = {:?}", v5);

    let v6 = vec![1, 2, 3, 13, 15, 17];
    let v7: Vec<_> = v6.into_iter().filter(|x| *x > 13).collect();
    println!("v2 = {:?}", v7);
    println!("Hello, world!");



    let mut counter = Counter::new();

    for i in 0..6 {
        if let Some(v) = counter.next(){
            println!("i = {}, v = {}", i, v);
        }else{
            println!("i = {}, at end", i);
            break;
        }
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
