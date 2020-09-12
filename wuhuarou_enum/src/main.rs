#[derive(Debug)]
enum IpAddKind {
    V4(String),
    V6(String),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddKind,
    address: String,
}

fn main() {
    let i1 = IpAddr {
        kind: IpAddKind::V4(String::from("ipv4")),
        address: String::from("127.0.0.1:8080"),
    };

    println!("i1 = {:?} ", i1);

    let i2 = IpAddr {
        kind: IpAddKind::V6(String::from("ipv6")),
        address: String::from("127.0.0.1:8080"),
    };

    println!("i2 = {:?} ", i2);

    let i3: i32 = 22;
    let sum: Option<i32> = None;
    println!("sum = {:?}, i3 = {}", sum, i3);
}
