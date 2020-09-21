fn main() {
    println!("Hello, world!");

    let stu = Student {
        name: String::from("hello world"),
        age: 10,
    };
    let lsu = Teacher {
        name: String::from("lao jiang"),
        age: 30,
        subject: String::from("消费"),
    };
    println!("学生姓名: {}, age: {}", stu.get_name(), stu.get_age());
    println!("学生姓名: {}, age: {} ", lsu.get_name(), lsu.get_age());
    print_information(stu);
    print_information(lsu);
}

pub trait GetInformation {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
}

pub struct Student {
    pub name: String,
    pub age: u32,
}

impl GetInformation for Student {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}

pub struct Teacher {
    pub name: String,
    pub age: u32,
    pub subject: String,
}

impl GetInformation for Teacher {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}


fn print_information(item: impl GetInformation){
    println!("name: {}", item.get_name());
    println!("age: {}", item.get_age());
}