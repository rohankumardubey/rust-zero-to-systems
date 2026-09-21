//! # Lifetime annotations
//!
//! Level: Intermediate
//! Category: Lifetimes and smart pointers
//!
//! ## What
//! A lifetime annotation states a relationship between borrowed inputs and borrowed output.
//!
//! ## Why
//! The compiler needs proof that a returned reference cannot outlive the data it refers to.
//!
//! ## When to use
//! Add annotations when a function returns a borrow selected from multiple input borrows.
//!
//! ## Avoid when
//! Do not add lifetimes to owned values or use `'static` to silence a relationship error.
//!
//! ## Prerequisites
//! Shared borrows and functions.
//!
//! ## Related
//! Lifetime elision, struct lifetimes, non-lexical lifetimes.
//!
//! ## Run
//! `cargo run -p lifetimes-smart-pointers-lessons --example 001_lifetime_annotations`

fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

fn main() {
    let left = String::from("durable");
    let right = String::from("safe");
    println!("{}", longer(&left, &right));
}

#[cfg(test)]
mod tests {
    use super::longer;

    #[test]
    fn returned_borrow_comes_from_an_input_borrow() {
        assert_eq!(longer("longer", "short"), "longer");
    }
}
