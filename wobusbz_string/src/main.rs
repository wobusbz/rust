use std::collections::HashMap;
fn main() {
    let mut sname = String::new();
    sname.push_str("hello world");
    println!("{}", sname);


    let user_info = UserInfo{
        name:String::from("wuhuarou"),
        age: 32,
    };
    user_info.get_user_name();
    user_info.get_user_age();
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let v4 = IpAddr::V4(String::from("127.0.0.1"));
    let v6 = IpAddr::V6(String::from("127.0.0.1"));
    println!("{:?}, {:?}", v4, v6);


    let mut  scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("wuhuarou"), 1);
    println!("{:?}", scores);
    println!("Hello, world!");
}

struct UserInfo {
    name: String,
    age: i32,
}

impl UserInfo {
    fn get_user_name(&self) {
        println!("{}", self.name)
    }
    fn get_user_age(&self) {
        println!("{}", self.age);
    }
}
