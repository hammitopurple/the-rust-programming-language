fn main() {
    println!("Hello, world!");
    another_function();
    function_with_param(55);
    function_with_many_params(22, 'l');

    let five = get_five();
    println!("the value of five is {five}");

    let four = get_four();
    println!("the value of four is {four}");

    let block = rust_block();
    println!("the value of block is {block}");

    if bigger_than_two(five) {
        println!("five is bigger than two!")
    }

    if false {
        infinite_loop();
    }

    let fib5 = fib(5);
    println!("this is the value of fib5: {fib5}");

    let (a, b, c) = multiple_let();
    println!("these are their new values: {a}, {b}, {c}");
}

fn another_function() {
    println!("another function!");
}

fn function_with_param(x: i32) {
    println!("the number is {x}")
}

fn function_with_many_params(x: i32, y: char) {
    println!("x is {x} and y is {y}")
}

fn get_five() -> i32 {
    return 5;
}

fn get_four() -> i32 {
    4
}

fn rust_block() -> i32 {
    let y = {
        let x = 3;
        x + 1
    };
    return y;
}

fn bigger_than_two(x: i32) -> bool {
    x > 2
}

fn infinite_loop() {
    loop {
        println!("this is an infinite loop!")
    }
}

fn fib(n: i32) -> i32 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    return a;
}

fn multiple_let() -> (u8, f32, i32) {
    let (mut a, mut b, mut c): (u8, f32, i32) = (8, 5.5, 99);
    println!("these are their values: {a}, {b}, {c}");

    // mutate the values
    a = 7;
    b = 4.5;
    c = 98;
    return (a, b, c);
}
