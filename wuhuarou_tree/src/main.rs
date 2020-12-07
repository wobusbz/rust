use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    child: RefCell<Vec<Rc<Node>>>,
}
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());


    let leaf1 = Rc::new(Node {
        value: 30,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![]),
    });
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf1), Rc::weak_count(&leaf));

    let branch1 = Rc::new(Node {
        value: 50,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![Rc::clone(&leaf1)]),
    });
    *leaf1.parent.borrow_mut() = Rc::downgrade(&branch1);
    println!("branch1 strong = {}, weak = {}", Rc::strong_count(&branch1), Rc::weak_count(&branch1));
    println!("Hello, world!");
    println!("{}", "hello world");
    
}
