//! # Move semantics
//!
//! Level: Beginner
//! Category: Ownership and memory
//!
//! ## What
//! Assigning or passing an owning value such as `String` moves ownership unless the type is `Copy`.
//!
//! ## Why
//! A move prevents two owners from freeing the same heap allocation.
//!
//! ## When to use
//! Move a value when the receiver should become responsible for it.
//!
//! ## Avoid when
//! Do not clone merely to satisfy the compiler before deciding which code should own the value.
//!
//! ## Prerequisites
//! Stack and heap values.
//!
//! ## Related
//! Borrowing, `Copy`, `Clone`, `Drop`.
//!
//! ## Run
//! `cargo run -p ownership-borrowing-lessons --example 002_move_semantics`

fn qualify(mut name: String) -> String {
    name.insert_str(0, "service::");
    name
}

fn main() {
    let name = String::from("catalog");
    let qualified = qualify(name);

    println!("{qualified}");
}

#[cfg(test)]
mod tests {
    use super::qualify;

    #[test]
    fn ownership_moves_into_and_out_of_the_function() {
        assert_eq!(qualify(String::from("search")), "service::search");
    }
}
