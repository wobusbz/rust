#[derive(Debug)]
enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

fn main() {

    let msg = Message::ChangeColor(0, 256, 256);

    match msg{
        Message::Quit => {
            println!("quit");
        },
        Message::Move{x,y} =>{
            println!("move, x: {}, y: {}", x ,y)
        },
        Message::Write(text) => {
            println!("write msg : {}", text);
        },
        Message::ChangeColor(r, g, b) => {
            println!("ChangeColor r: {} g: {} b: {}", r, g ,b);
        }
    };




    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("< 5"),
        Some(x) => println!("x : {}", x),
        None => (),
    };

    println!("num");
    println!("Hello, world!");


    let i = 5;
    let j = false;

    match i {
        4 | 5 | 6 if j => println!("1"),
        _ => println!("2"),
    }

    let msg_bak = Message_bak::Hello{id: 5};
    match msg_bak{
        Message_bak::Hello{id: id_va @ 3..=7} => {
            println!("id_va : {}", id_va);
        },
        Message_bak::Hello{id:10..=20} => {
            println!("large");
        },
        Message_bak::Hello{id} => {
            println!("id : {}", id);
        },
    }

}

#[derive(Debug)]
enum Message_bak {
    Hello{id : i32},
}
