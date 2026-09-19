//! # References and dereferencing
//!
//! Level: Beginner
//! Category: Core Rust
//!
//! ## What
//! `&T` borrows a value for reading, `&mut T` borrows it for mutation, and `*reference`
//! dereferences a reference to reach the referred-to value.
//!
//! ## Why
//! References let functions use or update values without taking ownership of them.
//!
//! ## When to use
//! Borrow with `&T` for read-only input and `&mut T` when a function must update a caller value.
//!
//! ## Avoid when
//! Do not reach for a reference to evade a necessary ownership transfer.
//!
//! ## Prerequisites
//! Variables, functions, and tuples.
//!
//! ## Related
//! Ownership, borrowing rules, slices, lifetimes.
//!
//! ## Run
//! `cargo run -p core-rust-lessons --example 016_references_and_dereference`

fn increment(value: &mut i32) {
    *value += 1;
}

fn main() {
    let mut retries = 2;
    let read_only = &retries;
    println!("before: {read_only}");

    increment(&mut retries);
    println!("after: {retries}");
}

#[cfg(test)]
mod tests {
    use super::increment;

    #[test]
    fn mutable_reference_updates_the_original_value() {
        let mut value = 41;
        increment(&mut value);
        assert_eq!(value, 42);
    }
}
