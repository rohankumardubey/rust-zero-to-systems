//! # Conversion and borrowing traits
//!
//! Level: Intermediate
//! Category: Generics and traits
//!
//! ## What
//! `Into<String>` accepts values that can be converted into owned text, while `AsRef<str>` accepts
//! values that can be viewed as text without taking ownership.
//!
//! ## Why
//! These standard traits let APIs support several compatible input types with clear ownership
//! behavior.
//!
//! ## When to use
//! Use `Into<T>` for flexible owned inputs and `AsRef<T>` for cheap borrowed views.
//!
//! ## Avoid when
//! Do not accept a generic conversion trait when the API needs one precise input type or must avoid
//! conversions.
//!
//! ## Prerequisites
//! Generics, ownership, and references.
//!
//! ## Related
//! `From`, `TryFrom`, `Borrow`, `Deref`, `AsMut`.
//!
//! ## Run
//! `cargo run -p types-generics-traits-lessons --example 006_conversion_and_borrowing_traits`

fn owned_message(value: impl Into<String>) -> String {
    value.into()
}

fn text_length(value: impl AsRef<str>) -> usize {
    value.as_ref().len()
}

fn main() {
    let message = owned_message("trait-powered API");
    println!("{message}: {} bytes", text_length(&message));
}

#[cfg(test)]
mod tests {
    use super::{owned_message, text_length};

    #[test]
    fn conversion_takes_ownership_while_as_ref_borrows() {
        let original = String::from("hello");
        assert_eq!(text_length(&original), 5);
        assert_eq!(owned_message(&original), "hello");
        assert_eq!(original, "hello");
    }
}
