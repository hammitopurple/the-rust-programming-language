use std::fmt::Display;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let origin = Point { x: 0, y: 0 };
    println!("coordinates: {}, {}", origin.x(), origin.y());

    let floating_point: Point<f32> = Point { x: 1.2, y: 9.0 };
    println!(
        "it is this far from origin: {}",
        floating_point.distance_from_origin()
    );

    let sms = SMS {
        number: 98765432,
        content: String::from("hello world!"),
    };
    println!("{}", sms.summarise());

    notify(&sms);

    let tweet = summarisable();
    println!("{}", tweet.summarise());

    let num_pair = Pair::new(2, 4);
    num_pair.cmp_display();

    let string_pair = Pair::new(String::from("hello"), String::from("world"));
    string_pair.cmp_display();

    let sms_2 = SMS {
        number: 12345678,
        content: String::from("hey!"),
    };
    let sms_pair = Pair::new(sms, sms_2);
    // sms_pair.cmp_display(); // error: the method `cmp_display` exists for struct `Pair<SMS>`, but its trait bounds were not satisfied
    println!("{}", sms_pair.y.summarise());

    print_displayable();

    let helloworld = String::from("hello world");
    let zawardo = String::from("za wardo!");
    println!("{}", longest(&helloworld, &zawardo));

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// note that these methods will only apply to Point<f32>!
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub trait Summary {
    fn summarise(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarise(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarise(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct SMS {
    pub number: i32,
    pub content: String,
}

impl Summary for SMS {}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarise())
}

fn summarisable() -> impl Summary {
    Tweet {
        username: String::from("User1"),
        content: String::from("This is the tweet's content"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn displayable<T: Display>(t: T) -> impl Display {
    t
}

fn print_displayable() {
    let s = String::from("hello");
    let s2 = displayable(s);
    // let mut s2 = displayable(s);
    // s2.push_str(" world"); // error: no method named `push_str` found for opaque type `impl std::fmt::Display` in the current scope
    println!("{s2}");
}

#[allow(dead_code, unused_variables)]
fn use_expired() {
    let r: &i32;
    {
        let x = 5;
        // r = &x; // error: `x` does not live long enough
    }
    let y = 55;
    r = &y;
    println!("{}", r)
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        y
    } else {
        x
    }
}

#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}
