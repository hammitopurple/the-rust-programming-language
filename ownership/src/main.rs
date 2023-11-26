fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");

    let a = Box::new(5);
    let b = a;
    // println!("{a}"); // this is not allowed because a has been moved
    println!("{b}");

    let mut m1 = String::from("hello");
    let m2 = String::from("world");
    greet(&mut m1, &m2);

    let number = Box::new(1);
    let foo = &number;
    let deref = dereference(foo); // actually why does this work? is &<T> == &Box<T> true?
    println!("this is deref: {deref}");

    // Testing permissions of paths
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    // v = vec![1, 2]; // not allowed!
    println!("the value of the 3rd index is {num}");
    v.push(4); // .push requires the W permission

    // Testing mut variables and references
    // let foo = 1;
    // let bar = &mut foo; // not allowed
    let mut foo = 1;
    let bar = &mut foo;
    *bar = 5;
    println!("this is the value of foo: {foo}");
    // let mut baz = &foo;
    // *baz = 3;

    // Testing mutable vector
    let mut s = vec![1, 2, 3];
    let x = &mut s;
    x.push(4);

    // Testing if can take ownership
    let name = vec![String::from("ferris")];
    let first_name = &name[0];
    take_and_return(&name);
    println!("{first_name}");

    // stringify implementation tests
    let george = vec![String::from("george")];
    let george_jr = stringify_name_with_title_with_clone(&george);
    let george_sr = stringify_name_with_title(&george);
    println!("their names are {} and {}", george_jr, george_sr);

    let jack = String::from("Jack");
    let dr_jack = award_phd(&jack);
    println!("look who got their phd: {}", dr_jack);

    let a = 'a';
    if a.is_numeric() {
        println!("wow it is");
    } else {
        println!("no it's not")
    }

    let mut string_vector = vec!["hello", "world"];
    let dst = &mut string_vector;
    let largest = dst.iter().max_by_key(|s| s.len()).unwrap();
    // dst.push("hello"); // error: cannot borrow `*dst` as mutable because it is also borrowed as immutable in "dst.iter()"
    println!("this is largest: {largest}");

    safe_tuple();
    unsafe_tuple();

    testing_slices();
}

fn add_suffix(mut name: String) -> String {
    // If name is not mutable, you will get error:
    // cannot borrow `name` as mutable, as it is not declared as mutable
    // cannot borrow as mutable
    name.push_str(" Jr.");
    name
}

fn greet(g1: &mut String, g2: &String) {
    g1.push('a');
    println!("{g1} {g2}!");
}

fn dereference(n1: &i32) -> i32 {
    *n1
}

fn take_and_return(thing: &Vec<String>) -> &Vec<String> {
    thing
}

fn stringify_name_with_title_with_clone(name: &Vec<String>) -> String {
    // This copies all of the data pointer to by name
    // Compared to the stringify_name_with_title method, it has an extra
    // clone step so in a sense it's a bit more wasteful in terms
    // of heap memory usage
    let mut name_clone = name.clone();
    name_clone.push(String::from(" Jr."));
    let full = name_clone.join(" ");
    full
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    // I think .join will dereference name to reach the vector,
    // then construct a String with " " as separators
    let mut full = name.join(" ");
    full.push_str(" Sr.");
    full
}

/// Returns a person's name with "Ph.D." added as a title
fn award_phd(name: &String) -> String {
    let mut name = name.clone();
    name.push_str(", Ph.D.");
    name
}

fn safe_tuple() {
    let mut name = (String::from("Ferris"), String::from("Rustacean"));
    let first = &name.0; // use . for accessing elements of tuple, [] is for vectors
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);
}

fn unsafe_tuple() {
    // let mut name = (String::from("Ferris"), String::from("Rustacean"));
    let name = (String::from("Ferris"), String::from("Rustacean"));
    let first = get_first(&name);
    // name.1.push_str(", Esq."); // error: cannot borrow `name.1` as mutable because it is also borrowed as immutable
    // println!("{first} {}", name.1);
    println!("Can't print the second element so we shall print the first: {first}");
}

#[allow(dead_code)]
fn get_first(tup: &(String, String)) -> &String {
    &tup.0
}

fn testing_slices() {
    // Taking a slice of an array
    let a = [1, 2, 3, 4, 5];
    let b = &a[..3];
    assert_eq!(b, &[1, 2, 3]);

    // Slices can work on collections too
    let c = vec![1, 2, 3, 4, 5];
    let d = &c[..3];
    assert_eq!(d, &[1, 2, 3]);

    // A slice of a String is of type &str
    let foo = String::from("lol");
    let bar = "lol";
    let baz = &foo[..];
    assert_eq!(bar, baz)
}

#[allow(dead_code)]
fn borrow_tests() {
    // Immutable borrow removes WO permissions
    let mut s = String::from("hello");
    let s_ref = &s;
    // s = String::from("bye"); // W perm removed from s: error: cannot assign to `s` because it is borrowed
    // let d = s; // O perm removed from s: error: cannot move out of `s` because it is borrowed
    println!("{s_ref}");

    // WO permissions are returned once s_ref is no longer is unse
    s = String::from("world");
    let mut d = s; // the move removes RWO perms from s
    println!("{d}");

    // Mutable borrow removes RWO permissions
    let d_ref_mut = &mut d;
    // let read_d = &d; // R perm removed from d: error: cannot borrow `d` as immutable because it is also borrowed as mutable

    // The reason why R perm is also removed from d is that d_ref_mut could invalidate the
    // heap data structure that d_ref_mut is pointing to e.g. d_ref_mut.push_str(...)
    // Once push_str occurs, the original heap location will be dropped and a new heap location
    // is allocated to store the newly-constructed string. read_d would therefore be
    // pointing to invalid memory
    d_ref_mut.push_str("string");
    println!("{d_ref_mut}");
}

#[allow(dead_code)]
fn use_after_free() {
    // If this code is allowed to compile, it will create a use-after-free problem
    // v.push will likely invalidate the heap data structure as another memory location
    // would need to be allocated to contain more elements within the vector
    // After v.push executes, the variable n's pointee would have been freed
    // This is an example of use-after-free

    // let mut v = vec![1, 2, 3]; // uncomment this if you want to see the error with .push()
    let v: Vec<i32> = vec![1, 2, 3];
    let n = &v[0];
    // v.push(4); // error: cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("{n}");
}

#[allow(dead_code)]
fn double_free() {
    // If this code is allowed to compile, it will create a double free problem
    // s_own and s point to the same heap data structure
    // When the function returns, s_own will be dropped first
    // When it comes to s, s will be dropped again, resulting in a double free

    let s = String::from("double");
    let s_ref = &s;
    // let s_own = *s_ref; // error: cannot move out of `*s_ref` which is behind a shared reference
    println!("{}", s_ref);
}
