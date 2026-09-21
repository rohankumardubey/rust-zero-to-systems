//! # `Box<T>`
//!
//! Level: Intermediate
//! Category: Lifetimes and smart pointers
//!
//! ## What
//! `Box<T>` owns a value allocated on the heap and provides exclusive access like an owned `T`.
//!
//! ## Why
//! It enables recursive data structures and moves large values by moving a pointer-sized handle.
//!
//! ## When to use
//! Use `Box` for recursive types, explicit heap ownership, or trait objects in later lessons.
//!
//! ## Avoid when
//! Do not box a value merely because it is inconvenient to pass; first understand ownership.
//!
//! ## Prerequisites
//! Ownership, moves, and enums.
//!
//! ## Related
//! `Rc`, `Arc`, deref coercion, recursive types.
//!
//! ## Run
//! `cargo run -p lifetimes-smart-pointers-lessons --example 004_box`

enum List {
    Node(i32, Box<List>),
    End,
}

fn sum(list: &List) -> i32 {
    match list {
        List::Node(value, next) => value + sum(next),
        List::End => 0,
    }
}

fn main() {
    let list = List::Node(1, Box::new(List::Node(2, Box::new(List::End))));
    println!("sum: {}", sum(&list));
}

#[cfg(test)]
mod tests {
    use super::{List, sum};

    #[test]
    fn box_makes_recursive_enum_size_finite() {
        let list = List::Node(40, Box::new(List::Node(2, Box::new(List::End))));
        assert_eq!(sum(&list), 42);
    }
}
