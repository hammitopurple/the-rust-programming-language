#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn set_width(&mut self, width: i32) {
        self.width = width
    }

    // max takes ownership of self
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 5,
        height: 8,
    };

    // Using function to calculate area of rectangle
    println!("The area of the rectangle is: {}", area(&rect1));
    println!("rect is {:?}", rect1); // normal print
    println!("rect is {:#?}", rect1); // pretty print

    // Using method to calculate area of rectangle
    println!("The area of the rectangle is {}", rect1.area());

    // Testing Rectangle's methods
    let rect2 = Rectangle {
        width: 3,
        height: 4,
    };
    let rect3 = Rectangle {
        width: 10,
        height: 20,
    };
    let rect4 = Rectangle {
        width: 1,
        height: 100,
    };
    println!("rect1 can hold rect1: {}", rect1.can_hold(&rect1));
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3));
    println!("rect1 can hold rect4: {}", rect1.can_hold(&rect4));

    // Method calls are syntactic sugar for function calls
    let mut rect5 = Rectangle {
        width: 5,
        height: 5,
    };
    println!("rect is {:#?}", rect5);
    rect5.set_width(10);
    println!("rect is {:#?}", rect5);
    Rectangle::set_width(&mut rect5, 5);
    println!("rect is {:#?}", rect5);

    let max_rect = rect1.max(rect2);
    // println!("{}", rect1.area()); // error: borrow of moved value: `rect1`
    println!("{}", max_rect.area())
}

fn area(rect: &Rectangle) -> i32 {
    rect.width * rect.height
}
