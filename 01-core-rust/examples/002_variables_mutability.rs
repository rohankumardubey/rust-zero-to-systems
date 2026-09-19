//! # Variables and mutability
//!
//! Level: Beginner
//! Category: Core Rust
//!
//! ## What
//! Bind names with `let`; add `mut` only when a binding must change.
//!
//! ## Why
//! Immutable-by-default bindings make accidental state changes visible in code.
//!
//! ## When to use
//! Use `let` for local values and `let mut` for a value whose binding changes.
//!
//! ## Avoid when
//! Do not use mutability as a substitute for modeling state transitions clearly.
//!
//! ## Prerequisites
//! Hello, world!
//!
//! ## Related
//! Shadowing, constants, ownership.
//!
//! ## Run
//! `cargo run -p core-rust-lessons --example 002_variables_mutability`

fn main() {
    let language = "Rust";
    let mut editions_seen = 1;
    editions_seen += 1;

    println!("{language} has {editions_seen} editions in this example.");
}

#[cfg(test)]
mod tests {
    #[test]
    fn mutable_binding_can_change() {
        let mut count = 1;
        count += 1;
        assert_eq!(count, 2);
    }
}
