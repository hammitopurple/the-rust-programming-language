pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    value: i32,
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.value + 1)
    }
}

pub trait IteratorGeneric<T> {
    fn follow(&mut self) -> Option<T>;
}

impl IteratorGeneric<i32> for Counter {
    fn follow(&mut self) -> Option<i32> {
        Some(self.value + 1)
    }
}

use std::ops::Add; // this is for the "+" operator

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point; // `Add` has an associated type that you have to provide a concrete type for

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}
struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms frantically*")
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", ")) // self.0 means the 0th-indexed item in the struct
    }
}

use std::collections::HashMap;

/// People is a tuple struct
struct People(HashMap<i32, String>);

impl People {
    /// add_name is the example public API as described in 19.3
    /// Although I'm not sure why you would even want to use i32 as a key
    fn add_name(&mut self, name: &str) -> Option<String> {
        self.0.insert(rand::random(), name.to_string())
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
#[allow(unused)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    let mut counter = Counter { value: 5 };
    println!("{:?}", counter.next());
    println!("{:?}", counter.follow());

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;
    person.fly(); // Can be rewritten as Human::fly(&person);
    Pilot::fly(&person);
    Wizard::fly(&person);

    println!("A baby dog is called {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let point = Point { x: 1, y: 3 };
    point.outline_print();

    let w = Wrapper(vec![String::from("Hello"), String::from("world")]);
    println!("w = {}", w);

    let mut people = People(HashMap::new());
    assert_eq!(people.add_name("user1"), None);
    assert_eq!(people.add_name("user1"), None);

    assert_eq!(do_twice(add_one, 1), 4);
    // let y = 5;
    // do_twice(|x| y + 1, 4); // error!

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{:?}", list_of_statuses);
}
