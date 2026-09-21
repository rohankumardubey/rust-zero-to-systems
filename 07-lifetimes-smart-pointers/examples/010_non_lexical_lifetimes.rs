//! # Non-lexical lifetimes
//!
//! Level: Intermediate
//! Category: Lifetimes and smart pointers
//!
//! ## What
//! Non-lexical lifetimes end a borrow at its last use, rather than always at the end of its block.
//!
//! ## Why
//! Safe code can reuse a value after a borrow is no longer needed without artificial nested scopes.
//!
//! ## When to use
//! Write direct code and let the compiler infer the smallest valid borrow region.
//!
//! ## Avoid when
//! Do not depend on subtle last-use behavior for readability; use a scope when it clarifies intent.
//!
//! ## Prerequisites
//! Shared and mutable borrows.
//!
//! ## Related
//! Reborrowing, lifetime annotations, borrow checker diagnostics.
//!
//! ## Run
//! `cargo run -p lifetimes-smart-pointers-lessons --example 010_non_lexical_lifetimes`

fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or_default()
}

fn main() {
    let mut command = String::from("build --release");
    let first = first_word(&command);
    println!("first token: {first}");
    command.push_str(" --locked");
    println!("updated command: {command}");
}

#[cfg(test)]
mod tests {
    use super::first_word;

    #[test]
    fn mutable_access_follows_the_last_use_of_shared_borrow() {
        let mut command = String::from("cargo test");
        assert_eq!(first_word(&command), "cargo");
        command.push_str(" --doc");
        assert_eq!(command, "cargo test --doc");
    }
}
