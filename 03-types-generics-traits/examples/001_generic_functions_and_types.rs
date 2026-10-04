//! # Generic functions and types
//!
//! Level: Beginner
//! Category: Generics and traits
//!
//! ## What
//! A type parameter lets a function or data type work with multiple concrete types while keeping
//! compile-time type checking.
//!
//! ## Why
//! Generics reuse logic without erasing the types and guarantees callers need.
//!
//! ## When to use
//! Use generics when the same behavior applies to multiple types with compatible operations.
//!
//! ## Avoid when
//! Do not add a type parameter when one concrete type expresses the intended API more clearly.
//!
//! ## Prerequisites
//! Functions, structs, references, and enums.
//!
//! ## Related
//! Trait bounds, monomorphization, `Option`, iterators.
//!
//! ## Run
//! `cargo run -p types-generics-traits-lessons --example 001_generic_functions_and_types`

#[derive(Debug, PartialEq, Eq)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }

    fn first(&self) -> &T {
        &self.first
    }
}

fn greatest<T: PartialOrd>(values: &[T]) -> Option<&T> {
    values
        .iter()
        .reduce(|left, right| if left < right { right } else { left })
}

fn main() {
    let pair = Pair::new("generic", "types");
    let numbers = [3, 8, 5];
    println!(
        "first: {}; greatest: {:?}",
        pair.first(),
        greatest(&numbers)
    );
}

#[cfg(test)]
mod tests {
    use super::{Pair, greatest};

    #[test]
    fn generic_type_and_function_work_for_concrete_types() {
        let pair = Pair::new(String::from("owned"), String::from("values"));
        assert_eq!(pair.first(), "owned");
        assert_eq!(greatest(&[4, 1, 9]), Some(&9));
        assert_eq!(greatest::<i32>(&[]), None);
    }
}
