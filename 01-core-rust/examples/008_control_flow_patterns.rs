//! # Control flow and patterns
//!
//! Level: Beginner
//! Category: Core Rust
//!
//! ## What
//! Rust has `if`, `loop`, `while`, `for`, `match`, `if let`, `while let`, and
//! `let else` for branching and repetition.
//!
//! ## Why
//! Pattern-aware control flow makes the handled shape of data explicit and encourages
//! exhaustive case analysis.
//!
//! ## When to use
//! Prefer `match` for multiple variants and `if let` for one interesting pattern.
//!
//! ## Avoid when
//! Do not use `loop` when a finite iterator communicates the work more directly.
//!
//! ## Prerequisites
//! Functions, `Option`, and arrays.
//!
//! ## Related
//! Enums, iterators, error handling.
//!
//! ## Run
//! `cargo run -p core-rust-lessons --example 008_control_flow_patterns`

fn describe(value: Option<i32>) -> String {
    let Some(number) = value else {
        return "no value".to_owned();
    };

    match number {
        0 => "zero".to_owned(),
        1..=9 => "single digit".to_owned(),
        _ if number < 0 => "negative".to_owned(),
        _ => "many digits".to_owned(),
    }
}

fn main() {
    let mut values = [Some(1), None, Some(3)].into_iter();
    let mut total = 0;

    while let Some(Some(value)) = values.next() {
        total += value;
    }

    for label in [describe(Some(total)), describe(None)] {
        println!("{label}");
    }
}

#[cfg(test)]
mod tests {
    use super::describe;

    #[test]
    fn match_covers_all_integer_categories() {
        assert_eq!(describe(Some(-1)), "negative");
        assert_eq!(describe(Some(7)), "single digit");
        assert_eq!(describe(None), "no value");
    }
}
