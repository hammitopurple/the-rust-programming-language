#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn create_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}

fn create_box() {
    let b = Box::new(5);
    println!("b = {}", b); // automatic dereference
    println!("b = {}", *b); // manual dereference
}

fn main() {
    create_box();
    create_list();
}
