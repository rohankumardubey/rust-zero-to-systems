//! # Functions, statements, expressions, and blocks
//!
//! Level: Beginner
//! Category: Core Rust
//!
//! ## What
//! Functions declare typed inputs and outputs. Expressions evaluate to values;
//! statements perform an action and usually end in a semicolon.
//!
//! ## Why
//! Expression-oriented blocks keep value construction close to the conditions that
//! determine it.
//!
//! ## When to use
//! Use a tail expression for a small computed return value and statements for effects.
//!
//! ## Avoid when
//! Do not omit a semicolon accidentally when a statement rather than a returned value is intended.
//!
//! ## Prerequisites
//! Variables and scalar types.
//!
//! ## Related
//! Control flow, closures, the never type.
//!
//! ## Run
//! `cargo run -p core-rust-lessons --example 007_functions_statements_expressions`

fn doubled(value: i32) -> i32 {
    value * 2
}

fn labeled_temperature(celsius: i32) -> &'static str {
    if celsius < 0 {
        "freezing"
    } else {
        "not freezing"
    }
}

fn main() {
    let answer = doubled(21);
    let label = labeled_temperature(-2);
    println!("{answer}; {label}");
}

#[cfg(test)]
mod tests {
    use super::{doubled, labeled_temperature};

    #[test]
    fn tail_expressions_become_return_values() {
        assert_eq!(doubled(4), 8);
        assert_eq!(labeled_temperature(0), "not freezing");
    }
}
