use std::collections::HashMap;

fn main() {
    let mut string_hash_map: HashMap<String, i32> = HashMap::new();
    string_hash_map.insert(String::from("hello world"), 1000);
    println!("{:?}", string_hash_map);

    let keys = vec![String::from("hello"), String::from("world")];
    let values = vec![1, 2];
    let scores: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

    println!("{:?}", scores);

    let v = scores.get(&String::from("hello"));

    match v {
        Some(values) => println!("v= {}", values),
        None => println!("NONE"),
    }

    for (key, val) in &scores {
        println!("key: {} val: {}\t", key, val);
    }

    // 插入
    let mut str_map = HashMap::new();
    str_map.insert(String::from("one"), 1);
    str_map.insert(String::from("two"), 2);
    str_map.insert(String::from("three"), 3);
    println!("{:?}", str_map);

    // 键值不存在则插入
    let mut str_map1 = HashMap::new();
    str_map1.insert(String::from("one"), 1);
    str_map1.insert(String::from("two"), 2);
    str_map1.insert(String::from("three"), 3);
    str_map1.entry(String::from("three")).or_insert(3); // 键值不存在则插入
    println!("{:?}", str_map1);

    // 根据旧值来更新一个新值
    let text = "hello world wonderful world";
    let mut list_map = HashMap::new();
    for word in text.split_ascii_whitespace() {
        let count = list_map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", list_map);
    println!("Hello, world!");
}
