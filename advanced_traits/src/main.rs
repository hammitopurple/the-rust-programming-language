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

fn main() {
    let mut counter = Counter { value: 5 };
    println!("{:?}", counter.next());
    println!("{:?}", counter.follow());
}
