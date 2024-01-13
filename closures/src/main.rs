use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn borrow_closure() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // The variable `list` is captured from the closure's environment
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn mutable_borrow_closure() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    // println!("{:?}", list); // error: cannot borrow `list` as immutable because it is also borrowed as mutable
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn move_closure() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
    move || s_ref.to_string()
}

fn iterator_non_mutable_reference() {
    let v = vec![1, 2, 3];

    // let v_iter = v.iter();
    // assert_eq!(v_iter.next(), Some(&1)); // error: cannot borrow `v_iter` as mutable, as it is not declared as mutable

    // Need to declare the iterator as mutable because .next() will change its internal state
    let mut v_iter = v.iter();
    assert_eq!(v_iter.next(), Some(&1));
}

fn consuming_adaptor() {
    // Demonstrate that thesum method consumes an iterator
    let v = vec![1, 2, 3];
    let v_iter = v.iter();

    let total: i32 = v_iter.sum();
    println!("the total is {total}");
    // println!("{:?}", v_iter); // error: borrow of moved value: `v_iter`
}

fn iterator_adaptor() {
    let v = vec![1, 2, 3];
    let v2: Vec<_> = v.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

fn main() {
    // Create the store with 2 blue shirts and 1 red shirt
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    borrow_closure();
    mutable_borrow_closure();
    move_closure();

    let s_own = String::from("new string");
    let cloner = make_a_cloner(&s_own);
    let bar = cloner();
    println!("{bar}");

    iterator_non_mutable_reference();
    consuming_adaptor();
    iterator_adaptor();
}
