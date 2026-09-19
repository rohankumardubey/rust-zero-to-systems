//! # Shared borrows
//!
//! Level: Beginner
//! Category: Ownership and memory
//!
//! ## What
//! A shared reference, `&T`, lets code read a value without taking ownership or mutating it.
//!
//! ## Why
//! APIs can inspect caller-owned data without allocation or transfer.
//!
//! ## When to use
//! Accept `&T` or a more specific borrowed view such as `&str` for read-only input.
//!
//! ## Avoid when
//! Do not request ownership when a temporary read is all the function needs.
//!
//! ## Prerequisites
//! Moves and references.
//!
//! ## Related
//! Mutable borrows, slices, lifetimes.
//!
//! ## Run
//! `cargo run -p ownership-borrowing-lessons --example 004_shared_borrows`

fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

fn main() {
    let message = String::from("ownership keeps resources clear");
    println!("{} words in: {message}", word_count(&message));
}

#[cfg(test)]
mod tests {
    use super::word_count;

    #[test]
    fn shared_borrow_leaves_owner_available() {
        let message = String::from("one two");
        assert_eq!(word_count(&message), 2);
        assert_eq!(message, "one two");
    }
}
