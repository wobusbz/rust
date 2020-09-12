#[derive(Debug)]
struct User {
    name: String,
    addr: String
}

impl User {
    fn get_name(&self) ->&str { &(self.name[..]) }
    
    fn get_addr(&self) ->&str { &(self.addr[..]) }
}

fn main() {

    let user = User{
        name: String::from("wuhaurou"),
        addr: String::from("中国")
    };
    println!("user -> {:?}", user);
    println!("user.name -> {}", user.get_name());
    println!("user.name -> {}", user.get_addr());
}
