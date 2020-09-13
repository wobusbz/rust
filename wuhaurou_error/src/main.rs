use std::fs::File;
use std::io;
use std::io::Read;
fn main() {
    let f = File::open("./a.txt");
    let _r = match f {
        Ok(file) => file,
        Err(error) => panic!("error:{:?}", error),
    };

    println!("Hello, world!");
    let r = read_username_form_file();
    match r {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("{:?}", e),
    }
    let r1 = read_username_form_file1();
    match r1 {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("{:?}", e),
    }
    let r2 = read_username_form_file2();
    match r2 {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("{:?}", e),
    }
}

fn read_username_form_file() -> Result<String, io::Error> {
    let f = File::open("./a.txts");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error),
    }
}

fn read_username_form_file1() -> Result<String, io::Error> {
    let mut f = File::open("./a.txts")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_form_file2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("a.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// (1) 示例、代码原型、测试用panic！\unwrap\expect
// (2) 实际项目中应该使用Result