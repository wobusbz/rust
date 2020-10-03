use std::fmt::Display;

fn longer_life<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() {
        x
    } else {
        y
    }
}
#[derive(Debug)]
struct User<'a> {
    name: &'a str,
}

impl<'a> User<'a> {
    fn do_something(&self) -> i32 {
        3
    }

    fn do_something2(&self) -> &str {
        self.name
    }
}

fn function<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("{}", ann);
    if x.len() < y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = String::from("abcde");
    let y = String::from("abc");
    let r = longer_life(x.as_str(), y.as_str());
    println!("{}", r);
    let name = String::from("wuhaurou_life");
    let user = User {
        name: name.as_str(),
    };
    println!("{:#?}", user);
    println!("{}", user.do_something());
    println!("{}", user.do_something2());

    let s1 = String::from("i am s1");
    let s2 = String::from("i am s2, hello world");
    let ann = 129;
    let r1 = function(s1.as_str(), s2.as_str(), ann);
    println!("{}", r1);
    println!("Hello, world!");
}
