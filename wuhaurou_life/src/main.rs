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

fn main() {
    let x = String::from("abcde");
    let y = String::from("abc");
    let r = longer_life(x.as_str(), y.as_str());
    println!("{}", r);
    let name = String::from("wuhaurou_life");
    let user = User{
        name: name.as_str(),
    };
    println!("{:#?}", user);
    println!("Hello, world!");
}
