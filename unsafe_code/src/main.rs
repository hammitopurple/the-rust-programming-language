static mut COUNTER: u32 = 0;

fn create_raw_pointers() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("value of r1 is {}", *r1);
        println!("value of r2 is {}", *r2);
        dangerous();
    }
}

unsafe fn dangerous() {
    println!("this is some dangerous code");
}

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

struct Bar {}

unsafe trait Foo {
    unsafe fn hi(&self);
}

unsafe impl Foo for Bar {
    unsafe fn hi(&self) {
        println!("hi");
    }
}

fn main() {
    create_raw_pointers();
    add_to_counter(1);
    unsafe { println!("COUNTER is: {}", COUNTER) }

    let bar = Bar {};
    unsafe {
        bar.hi();
    };
}
