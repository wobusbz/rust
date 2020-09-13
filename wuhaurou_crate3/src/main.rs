extern crate crypto; // 外部库引入
use crypto::digest::Digest;
use crypto::sha3::Sha3;

fn main() {
    let mut h = Sha3::sha3_256();
    h::input_str("hello world");
    let result = h.result_str();
    println!("hash = {}", result);
    println!("Hello, world!");
}
