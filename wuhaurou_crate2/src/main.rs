use game::factory::produce_refrigerators as a;
use game::factory::produce_washing_machine as b;


mod mod_a {
    #[derive(Debug)]
    pub struct A{
        pub number: i32,
        name: String
    }

    impl A {
        pub fn new_a() -> A{
            A{
                number:1,
                name:String::from("A"),
            }
        }

        pub fn print_a(&self) {
            println!("number: {}, name: {}", self.number, self.name);
        }
    }

    pub mod mod_b{
        pub fn print_b() {
            println!("b");
        }

        pub mod mod_c{
            pub fn print_c() {
                super::print_b();    
                println!("c");
            }
        }
    }
}
fn main() {
    a::produce_re();
    b::produce_washing_machine();

    let a = mod_a::A::new_a();
    a.print_a();

    use mod_a::A;
    let b = A::new_a();
    b.print_a();
    let number = b.number;
    println!("{}", number);
    mod_a::mod_b::mod_c::print_c();
}
