use cargo_features::{add_one, return_two};

/// return_one returns 1
///
/// # Examples
/// ```
/// let bar = baz();
/// assert_eq!(1, bar);
/// ```
fn return_one() -> i32 {
    1
}

fn main() {
    println!("hey there");
    let foo = add_one(5);
    println!("the value of foo is: {foo}");
    let bar = return_one();
    println!("{bar}");
    let baz = return_two();
    println!("{baz}")
}
