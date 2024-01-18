//! # Header 1
//!
//! `cargo_features` is my create
//!
//! # Header 2
//!
//! I can add new sections!

// Declare module
pub mod utils;

// Re-exported functions
pub use utils::re_exported_fn;

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = cargo_features::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// return_two returns 2
///
/// # Examples
/// ```
/// let bar = cargo_features::return_two();
///
/// assert_eq!(2, bar);
/// ```
pub fn return_two() -> i32 {
    2
}
