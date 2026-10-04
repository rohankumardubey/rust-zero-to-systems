//! # Trait bounds and `where` clauses
//!
//! Level: Intermediate
//! Category: Generics and traits
//!
//! ## What
//! A trait bound restricts a generic parameter to types that provide the required behavior; a
//! `where` clause writes those requirements separately from the item signature.
//!
//! ## Why
//! Bounds make generic requirements explicit and let the compiler type-check operations used by
//! the implementation.
//!
//! ## When to use
//! Add the smallest set of bounds needed by the function or type.
//!
//! ## Avoid when
//! Avoid bounds that are not used, since they needlessly restrict callers.
//!
//! ## Prerequisites
//! Generic functions and trait implementations.
//!
//! ## Related
//! `Display`, `Debug`, blanket implementations, `impl Trait`.
//!
//! ## Run
//! `cargo run -p types-generics-traits-lessons --example 003_trait_bounds_and_where`

use std::fmt::{Debug, Display};

fn report<T>(value: &T) -> String
where
    T: Display + Debug,
{
    format!("shown as {value}; inspected as {value:?}")
}

fn main() {
    println!("{}", report(&42));
}

#[cfg(test)]
mod tests {
    use super::report;

    #[test]
    fn where_clause_documents_all_required_capabilities() {
        assert_eq!(report(&42), "shown as 42; inspected as 42");
    }
}
