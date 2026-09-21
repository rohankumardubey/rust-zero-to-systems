//! # `Cow<'a, T>`
//!
//! Level: Intermediate
//! Category: Lifetimes and smart pointers
//!
//! ## What
//! `Cow` is either a borrowed value or an owned value, allocating only when a transformation
//! needs ownership.
//!
//! ## Why
//! Read-mostly APIs can avoid clones on the unchanged path while returning one uniform type.
//!
//! ## When to use
//! Use `Cow` when an operation commonly returns its input but occasionally must create a modified
//! owned value.
//!
//! ## Avoid when
//! Do not use `Cow` if always borrowing or always owning makes the API clearer.
//!
//! ## Prerequisites
//! Borrowing, `String`, and lifetimes.
//!
//! ## Related
//! `ToOwned`, `AsRef`, allocation behavior.
//!
//! ## Run
//! `cargo run -p lifetimes-smart-pointers-lessons --example 009_cow`

use std::borrow::Cow;

fn normalize_ascii(input: &str) -> Cow<'_, str> {
    if input.bytes().any(|byte| byte.is_ascii_uppercase()) {
        Cow::Owned(input.to_ascii_lowercase())
    } else {
        Cow::Borrowed(input)
    }
}

fn main() {
    for input in ["stable", "Rust"] {
        println!("{input} -> {}", normalize_ascii(input));
    }
}

#[cfg(test)]
mod tests {
    use super::{Cow, normalize_ascii};

    #[test]
    fn unchanged_input_stays_borrowed_and_changed_input_is_owned() {
        assert!(matches!(normalize_ascii("stable"), Cow::Borrowed("stable")));
        assert_eq!(normalize_ascii("Rust"), "rust");
    }
}
