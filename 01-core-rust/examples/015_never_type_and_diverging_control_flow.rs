//! # The never type and diverging control flow
//!
//! Level: Intermediate
//! Category: Core Rust
//!
//! ## What
//! A function returning `!` never returns normally. `panic!`, `loop {}`, process exit, and an
//! early return can all diverge from the current control-flow path.
//!
//! ## Why
//! The type system can accept a diverging branch wherever a value would otherwise be needed.
//!
//! ## When to use
//! Use a diverging helper only for unrecoverable invariants or intentionally non-returning work.
//!
//! ## Avoid when
//! Prefer `Result` for expected failures that callers can handle.
//!
//! ## Prerequisites
//! Functions, `match`, and `Result` vocabulary.
//!
//! ## Related
//! Panics, error handling, exhaustive matching.
//!
//! ## Run
//! `cargo run -p core-rust-lessons --example 015_never_type_and_diverging_control_flow`

fn require_positive(value: i32) -> i32 {
    if value > 0 { value } else { fail(value) }
}

fn fail(value: i32) -> ! {
    panic!("expected a positive value, received {value}");
}

fn main() {
    println!("{}", require_positive(42));
}

#[cfg(test)]
mod tests {
    use super::require_positive;

    #[test]
    fn positive_value_returns_normally() {
        assert_eq!(require_positive(1), 1);
    }

    #[test]
    #[should_panic(expected = "expected a positive value")]
    fn non_positive_value_diverges_by_panicking() {
        let _ = require_positive(0);
    }
}
