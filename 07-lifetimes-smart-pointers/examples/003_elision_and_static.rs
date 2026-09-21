//! # Lifetime elision and `'static`
//!
//! Level: Intermediate
//! Category: Lifetimes and smart pointers
//!
//! ## What
//! Elision rules infer common lifetime relationships. A `&'static str` refers to text valid for
//! the entire program, often a string literal.
//!
//! ## Why
//! Common borrowed APIs stay readable while explicit annotations remain available for ambiguity.
//!
//! ## When to use
//! Rely on elision when one input borrow clearly determines the output; use `'static` only when
//! the data genuinely lives for the whole program.
//!
//! ## Avoid when
//! Do not confuse a `T: 'static` bound with a requirement to leak memory.
//!
//! ## Prerequisites
//! Lifetime annotations and borrowed strings.
//!
//! ## Related
//! Struct lifetimes, trait bounds, static items.
//!
//! ## Run
//! `cargo run -p lifetimes-smart-pointers-lessons --example 003_elision_and_static`

const DEFAULT_PROFILE: &str = "development";

fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or_default()
}

fn main() {
    let command = String::from("build --release");
    println!("{} ({DEFAULT_PROFILE})", first_word(&command));
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_PROFILE, first_word};

    #[test]
    fn elision_connects_output_to_the_single_input_borrow() {
        assert_eq!(first_word("cargo test"), "cargo");
        assert_eq!(DEFAULT_PROFILE, "development");
    }
}
