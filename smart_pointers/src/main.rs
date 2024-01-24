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

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn deref() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn hello_too(name: &String) {
    println!("Hello too, {name}!");
}

fn main() {
    create_box();
    create_list();

    deref();
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // 2 applications of deref coercion: &MyBox to &String, then &String to &str
    hello_too(&m); // vanilla case 1 of deref coercion, &T to &U

    let b = Box::new(String::from("rust"));
    hello(&b);
}
