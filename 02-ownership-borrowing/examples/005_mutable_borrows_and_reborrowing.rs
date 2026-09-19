//! # Mutable borrows and reborrowing
//!
//! Level: Intermediate
//! Category: Ownership and memory
//!
//! ## What
//! `&mut T` grants temporary exclusive access. Passing it onward creates a shorter reborrow,
//! not a second simultaneous mutable owner.
//!
//! ## Why
//! Exclusive access prevents unsynchronized conflicting mutation in safe Rust.
//!
//! ## When to use
//! Use a mutable borrow when a helper must update a caller-owned value in place.
//!
//! ## Avoid when
//! Do not keep mutable borrows alive longer than necessary; narrow their scope instead.
//!
//! ## Prerequisites
//! Shared borrows and functions.
//!
//! ## Related
//! Non-lexical lifetimes, interior mutability, `Mutex`.
//!
//! ## Run
//! `cargo run -p ownership-borrowing-lessons --example 005_mutable_borrows_and_reborrowing`

fn append_suffix(text: &mut String, suffix: &str) {
    text.push_str(suffix);
}

fn normalize(text: &mut String) {
    let reborrow: &mut String = text;
    append_suffix(reborrow, "!");
}

fn main() {
    let mut status = String::from("ready");
    normalize(&mut status);
    println!("{status}");
}

#[cfg(test)]
mod tests {
    use super::normalize;

    #[test]
    fn reborrow_has_a_shorter_use_scope() {
        let mut status = String::from("ready");
        normalize(&mut status);
        assert_eq!(status, "ready!");
    }
}
