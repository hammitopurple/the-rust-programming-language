#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

enum Location {
    Point(i32),
    Range(i32, i32),
}
fn print_range_max(loc: &Location) {
    // print the second field of Range, if loc is a Range
    if let Location::Range(_, y) = loc {
        println!("{y}");
    }
}

fn get_start(loc: &Location) -> i32 {
    // return the first field of Range or the only field of Point
    match loc {
        Location::Point(s) => *s,
        Location::Range(x, _) => *x,
    }
}

fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter(UsState::Alaska);
    println!("the penny is worth {} cents", value_in_cents(&penny));
    println!("the penny is worth {} cents", value_in_cents(&nickel));
    println!("the penny is worth {} cents", value_in_cents(&dime));
    println!("the penny is worth {} cents", value_in_cents(&quarter));

    // Quiz
    let point = Location::Point(10);
    let loc = Location::Range(1, 5);
    print_range_max(&loc);
    println!("{}", get_start(&point));
    println!("{}", get_start(&loc));

    // Option
    let some = Some(3);
    let none: Option<i32> = None;
    assert_eq!(match_things(some), 3);
    assert_eq!(match_things(none), 0);
}

fn match_things(opt: Option<i32>) -> i32 {
    match opt {
        Some(x) => x,
        None => 0,
    }
}
