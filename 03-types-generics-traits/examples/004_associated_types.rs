//! # Associated types
//!
//! Level: Intermediate
//! Category: Generics and traits
//!
//! ## What
//! An associated type is selected by a trait implementation, giving the trait one related type
//! without making every use choose an extra type parameter.
//!
//! ## Why
//! It models a one-to-one relationship between an implementing type and a type used by its API.
//!
//! ## When to use
//! Use an associated type when each implementation naturally has one output or item type.
//!
//! ## Avoid when
//! Use a generic trait parameter when callers should be able to implement the trait multiple times
//! for different type combinations.
//!
//! ## Prerequisites
//! Traits, generics, and references.
//!
//! ## Related
//! `Iterator::Item`, generic trait parameters, generic associated types.
//!
//! ## Run
//! `cargo run -p types-generics-traits-lessons --example 004_associated_types`

trait Source {
    type Item;

    fn next_item(&mut self) -> Option<Self::Item>;
}

struct NumberSource {
    next: u32,
    end: u32,
}

impl Source for NumberSource {
    type Item = u32;

    fn next_item(&mut self) -> Option<Self::Item> {
        if self.next == self.end {
            return None;
        }

        let current = self.next;
        self.next += 1;
        Some(current)
    }
}

fn main() {
    let mut source = NumberSource { next: 1, end: 4 };
    println!("next item: {:?}", source.next_item());
}

#[cfg(test)]
mod tests {
    use super::{NumberSource, Source};

    #[test]
    fn implementation_chooses_its_item_type() {
        let mut source = NumberSource { next: 7, end: 9 };
        assert_eq!(source.next_item(), Some(7));
        assert_eq!(source.next_item(), Some(8));
        assert_eq!(source.next_item(), None);
    }
}
