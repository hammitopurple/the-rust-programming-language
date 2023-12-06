use std::collections::HashMap;
use std::ops::Range;
use std::slice::Iter;

fn main() {
    for_loop();
    slice_iter();
    range_iter();
    mut_ref();
    hash_map();
    hash_map_move();
    add_key_if_not_present();
}

fn for_loop() {
    let v: Vec<i32> = vec![1, 2, 3];

    for n_ref in v {
        println!("{n_ref}");
    }
}

fn slice_iter() {
    let v: Vec<i32> = vec![1, 2, 3];

    let mut iter: Iter<'_, i32> = v.iter();
    let n1: &i32 = iter.next().unwrap();
    let n2: &i32 = iter.next().unwrap();
    let end: Option<&i32> = iter.next();
    println!("{n1}, {n2}");

    match end {
        Some(n) => println!("{n}"),
        None => println!("reached the end of the array"),
    }
}

fn range_iter() {
    let v: Vec<i32> = vec![1, 2, 3];

    let mut iter: Range<usize> = 0..v.len();
    let i1: usize = iter.next().unwrap();
    let n1: &i32 = &v[i1];
    println!("{n1}");
}

fn mut_ref() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let mut v2: Vec<&mut i32> = Vec::new();
    for i in &mut v {
        v2.push(i);
    }
    *v2[0] = 5;

    // Example to show you that ordering matters due to Rust's borrow checker
    // You need to let v2 be no longer in use first before you can access v again
    // That is because v2 contains mutable references to v
    // let b = v[0]; // error: cannot borrow `v` as immutable because it is also borrowed as mutable
    let a = *v2[0];
    let b = v[0];

    println!("{a} {b}");
}

fn hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 5);
    scores.insert(String::from("red"), 7);

    let team_name = String::from("blue");
    let score = scores.get(&team_name);

    match score {
        Some(v) => println!("they scored {v} points"),
        None => println!("they didn't score any points"),
    }
    let unwrapped_score = score.unwrap();
    println!("{unwrapped_score}");
}

fn hash_map_move() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // Use a reference to map, instead of map directly, or else
    // you will move the ownership, and can't use map anymore
    for (key, value) in &map {
        println!("my {key} is {value}")
    }
    // println!("this value is moved {field_name}, {field_value}") // error: borrow of moved value
}

fn add_key_if_not_present() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50); // inserts "Yellow":50
    scores.entry(String::from("Blue")).or_insert(50); // does nothing

    println!("{:?}", scores);
}
