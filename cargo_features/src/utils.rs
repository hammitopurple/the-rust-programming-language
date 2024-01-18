//! # Header for the utils module
//!
//! This is the utils module. Note that only public functions will appear
//! in the documentation.

/// Returns the "hello" String
pub fn hello() -> String {
    String::from("hello!")
}

#[allow(dead_code)]
fn bye() -> String {
    String::from("bye")
}

/// # Re-exported function!
///
/// This is so cool! I don't have to imported as cargo_features::utils::re_exported_fn,
/// I can be imported as cargo_features::re_exported_fn now
pub fn re_exported_fn() -> i32 {
    1
}
