//! # `str`, `&str`, and `String`
//!
//! Level: Beginner
//! Category: Core Rust
//!
//! ## What
//! `str` is UTF-8 text behind a pointer, `&str` borrows text, and `String` owns
//! growable UTF-8 text.
//!
//! ## Why
//! Separating borrowed and owned text lets APIs avoid allocation when callers already
//! have valid text to share.
//!
//! ## When to use
//! Accept `&str` for read-only text input; return or store `String` when ownership is needed.
//!
//! ## Avoid when
//! Do not index a string by integer offset: UTF-8 characters have variable byte length.
//!
//! ## Prerequisites
//! Slices and ownership vocabulary.
//!
//! ## Related
//! `AsRef<str>`, `Cow`, bytes, Unicode scalar values.
//!
//! ## Run
//! `cargo run -p core-rust-lessons --example 006_str_and_string`

fn greeting(target: &str) -> String {
    format!("Hello, {target}!")
}

fn main() {
    let borrowed: &str = "Rust";
    let mut owned = greeting(borrowed);
    owned.push('🦀');

    println!("{owned}");
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn borrowed_input_can_produce_owned_output() {
        assert_eq!(greeting("systems"), "Hello, systems!");
    }
}
