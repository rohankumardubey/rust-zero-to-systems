//! # `Cell<T>`
//!
//! Level: Intermediate
//! Category: Lifetimes and smart pointers
//!
//! ## What
//! `Cell<T>` allows replacement or copying of a value through a shared reference without creating
//! borrowed guards.
//!
//! ## Why
//! It supports simple interior mutation for `Copy` values while preserving aliasing guarantees.
//!
//! ## When to use
//! Use `Cell` for small `Copy` state such as counters or flags in single-threaded code.
//!
//! ## Avoid when
//! Use `RefCell` for non-`Copy` values or a synchronization primitive for cross-thread state.
//!
//! ## Prerequisites
//! Shared borrows and interior mutability vocabulary.
//!
//! ## Related
//! `RefCell`, `UnsafeCell`, atomics.
//!
//! ## Run
//! `cargo run -p lifetimes-smart-pointers-lessons --example 008_cell`

use std::cell::Cell;

fn increment(counter: &Cell<u32>) {
    counter.set(counter.get() + 1);
}

fn main() {
    let attempts = Cell::new(0);
    increment(&attempts);
    println!("attempts: {}", attempts.get());
}

#[cfg(test)]
mod tests {
    use super::{Cell, increment};

    #[test]
    fn shared_reference_can_replace_copy_value() {
        let counter = Cell::new(41);
        increment(&counter);
        assert_eq!(counter.get(), 42);
    }
}
