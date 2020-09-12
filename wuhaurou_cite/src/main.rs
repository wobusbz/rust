// rust 通过所有权机制来管理内存，编译器在变异就会根据所有权规则对内存的使用进行检查
// 堆 和 栈
// 编译的时候数据的类型大小是固定的，就是分配在栈上的
// 编译的时候数据类型大小不固定，就是分配堆上的
// 作用域 : {}
// string 内存回收
// 移动
// clone 
// 栈上数据拷贝
// 函数和作用域
fn main() {

    let x: u32 = 1217;

    {
        let x = "wuhuarou";
        println!("x = {}", x);
    }

    println!("x = {}", x);


    {
        let mut s1 = String::from("hello ");
        s1.push_str("wuhuarou");
        println!("x = {}", s1); // String 类型离开作用域的时候会调用drop方法


        let  s2 = String::from("hello ");
        println!("x = {}", s2); // String 类型离开作用域的时候会调用drop方法

        let s3 = &s2;
        println!("x = {}", s3);
        println!("x = {}", s2);
       
        let s4 = s3.clone();
        println!("s2 = {} ", s4);
    }

    let s = String::from("wo ai huarou a ");
    println!("x = {}",  fn1(s));

    let i = 5;
    fn2(i);
    println!("x = {}", i);
    println!("hello world");

    let str: String = String::from("Hello World");
    let len = get_string_len(&str);
    println!("len = {}", len);



    let  str1 = String::from("hello world");

    let w = &str1[..5];
    println!("{}", w);

    let int_arr = [1,2,3,4];
    println!("{:?}", int_arr);

    let user = User{
        name: String::from("wuhuarou"),
        age: 22,
        addr: String::from("重庆市天星桥")
    };

    println!("user = {:?}", user);

    let p = Point{x: 3,y: 5};
    println!("{:?}", p);//直接打印整个结构体，因为已经实现Debug，会成功
}
#[derive(Debug)]
struct Point{ //自定义一个结构体
    x: i32,
    y: i32
}
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    addr: String
}



fn fn1 (some: String) -> String{
    println!("x = {}", some);
    some
}

fn fn2(i: u32) {
    println!("x = {}", i);
    println!("");
}


fn get_string_len(str: &String) -> usize {
    str.len()
}
