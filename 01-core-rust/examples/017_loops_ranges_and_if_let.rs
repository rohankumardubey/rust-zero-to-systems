//! # Loops, ranges, and `if let`
//!
//! Level: Beginner
//! Category: Core Rust
//!
//! ## What
//! `loop` repeats until `break`, `while` repeats while a condition is true, ranges describe
//! ordered values, and `if let` handles one pattern succinctly.
//!
//! ## Why
//! Each form describes a distinct control-flow shape without manual jumps or sentinel values.
//!
//! ## When to use
//! Use `for` with ranges or iterators for finite work; use `while` and `loop` for condition- or
//! event-driven repetition.
//!
//! ## Avoid when
//! Do not hide a simple finite traversal inside an unconstrained `loop`.
//!
//! ## Prerequisites
//! Functions, expressions, and pattern matching.
//!
//! ## Related
//! Iterators, `while let`, `let else`, `match`.
//!
//! ## Run
//! `cargo run -p core-rust-lessons --example 017_loops_ranges_and_if_let`

fn sum_to(limit: u32) -> u32 {
    let mut total = 0;
    let mut current = 0;

    while current <= limit {
        total += current;
        current += 1;
    }

    total
}

fn main() {
    let mut attempts = 0;
    let result = loop {
        attempts += 1;
        if attempts == 3 {
            break attempts;
        }
    };

    if let Some(last) = (1..=result).last() {
        println!("attempt {last}; sum: {}", sum_to(last));
    }
}

#[cfg(test)]
mod tests {
    use super::sum_to;

    #[test]
    fn while_loop_visits_an_inclusive_range() {
        assert_eq!(sum_to(4), 10);
    }
}
