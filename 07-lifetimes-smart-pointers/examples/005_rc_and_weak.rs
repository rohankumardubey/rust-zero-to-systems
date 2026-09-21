//! # `Rc<T>` and `Weak<T>`
//!
//! Level: Intermediate
//! Category: Lifetimes and smart pointers
//!
//! ## What
//! `Rc<T>` shares ownership within one thread; `Weak<T>` observes an allocation without keeping
//! it alive.
//!
//! ## Why
//! Reference counting supports shared immutable graphs while weak references avoid ownership cycles.
//!
//! ## When to use
//! Use `Rc` for single-threaded shared ownership and `Weak` for non-owning back references.
//!
//! ## Avoid when
//! Do not use `Rc` across threads; use `Arc` in the concurrency section instead.
//!
//! ## Prerequisites
//! Ownership, `Box`, and shared borrows.
//!
//! ## Related
//! `Arc`, `RefCell`, cycles, strong and weak counts.
//!
//! ## Run
//! `cargo run -p lifetimes-smart-pointers-lessons --example 005_rc_and_weak`

use std::rc::{Rc, Weak};

fn main() {
    let owner = Rc::new(String::from("configuration"));
    let reader = Rc::clone(&owner);
    let observer: Weak<String> = Rc::downgrade(&owner);

    println!(
        "{reader}; strong: {}; observer alive: {}",
        Rc::strong_count(&owner),
        observer.upgrade().is_some()
    );
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    #[test]
    fn weak_reference_does_not_keep_value_alive() {
        let weak = {
            let owner = Rc::new(String::from("temporary"));
            Rc::downgrade(&owner)
        };

        assert!(weak.upgrade().is_none());
    }
}
