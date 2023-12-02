#[allow(dead_code)]
fn take_order() {
    // Absolute path
    crate::front_of_house::hosting::seat_at_table()
}

#[allow(dead_code)]
fn serve_order() {
    // Relative path
    super::hosting::seat_at_table()
}

#[allow(dead_code)]
pub fn take_payment() {
    // Cannot use a private function
    // super::hosting::private_hosting() // error: function `private_hosting` is private
}
