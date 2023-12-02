pub fn add_to_waitlist() {}

pub fn seat_at_table() {}

#[allow(dead_code)]
fn private_hosting() {
    // A submodule can use its ancestor's code
    crate::eat_breakfast();
}
