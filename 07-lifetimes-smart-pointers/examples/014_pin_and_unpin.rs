//! # `Pin` and `Unpin`
//!
//! Level: Advanced
//! Category: Lifetimes and smart pointers
//!
//! ## What
//! `Pin<P>` prevents moving a `!Unpin` value through that pinned pointer; `Unpin` types keep their
//! ordinary move behavior.
//!
//! ## Why
//! Some values, notably compiler-generated async state machines, rely on a stable address after
//! being pinned.
//!
//! ## When to use
//! Use pinning when an API's safety contract requires address stability, usually through an
//! established safe abstraction.
//!
//! ## Avoid when
//! Do not add pinning to ordinary owned data; it adds API constraints without benefit there.
//!
//! ## Prerequisites
//! Ownership, smart pointers, and auto traits.
//!
//! ## Related
//! `Future`, `PhantomPinned`, pin projection, async state machines.
//!
//! ## Run
//! `cargo run -p lifetimes-smart-pointers-lessons --example 014_pin_and_unpin`

use std::{marker::PhantomPinned, pin::Pin};

#[derive(Debug)]
struct AddressSensitive {
    label: String,
    _pin: PhantomPinned,
}

fn main() {
    let pinned = Box::pin(AddressSensitive {
        label: String::from("fixed while pinned"),
        _pin: PhantomPinned,
    });
    inspect(pinned.as_ref());

    let movable = Box::pin(String::from("String is Unpin"));
    let mutable = Pin::into_inner(movable);
    println!("moved out after unpin-compatible ownership: {mutable}");
}

fn inspect(value: Pin<&AddressSensitive>) {
    println!("pinned value: {}", value.label);
}

#[cfg(test)]
mod tests {
    use super::{AddressSensitive, inspect};
    use std::pin::Pin;

    #[test]
    fn pinned_borrow_can_read_a_not_unpin_value() {
        let value = Box::pin(AddressSensitive {
            label: String::from("stable address contract"),
            _pin: std::marker::PhantomPinned,
        });
        let pinned: Pin<&AddressSensitive> = value.as_ref();
        assert_eq!(pinned.label, "stable address contract");
        inspect(pinned);
    }

    #[test]
    fn unpin_value_can_be_recovered_from_pin_box() {
        let value = Box::pin(String::from("movable"));
        let value = Pin::into_inner(value);
        assert_eq!(value.as_str(), "movable");
    }
}
